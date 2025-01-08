//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;
use leafwing_input_manager::prelude::*;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, Smoother};

pub mod level;
pub mod critters;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct PlayerCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct NeedsTnua;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Jump,
}

#[derive(Resource)]
struct PlayerInputMap(InputMap<PlayerAction>);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        critters::plugin,
    ));
    app.register_type::<Player>();
    app.register_type::<PlayerCamera>();
    app.register_type::<NeedsTnua>();
    app.add_systems(Update, (
        setup_camera,
        maintain_camera,
        setup_tnua,
    ));
    app.add_systems(
        FixedUpdate,
        apply_controls.in_set(TnuaUserControlsSystemSet),
    );
    app.insert_resource(PlayerInputMap(InputMap::new([
        (PlayerAction::Up, KeyCode::ArrowUp),
        (PlayerAction::Down, KeyCode::ArrowDown),
        (PlayerAction::Left, KeyCode::ArrowLeft),
        (PlayerAction::Right, KeyCode::ArrowRight),
        (PlayerAction::Jump, KeyCode::Space),
    ])));
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
}

fn setup_camera(
    mut commands: Commands,
    camera: Query<(Entity, &GlobalTransform), (With<PlayerCamera>, Without<LookTransform>)>,
) {
    for (camera_entity, transform) in &camera {
        commands.entity(camera_entity).insert(
            LookTransformBundle {
                transform: LookTransform::new(transform.translation(), Vec3::default(), Vec3::Y),
                smoother: Smoother::new(0.9),
            }
        );
    }
}

fn maintain_camera(
    player: Query<&GlobalTransform, With<Player>>,
    mut camera: Query<&mut LookTransform>,
) {
    if let Ok(player_transform) = player.get_single() {
        for mut look_transform in &mut camera {
            look_transform.target = player_transform.translation();
            look_transform.eye = player_transform.translation() + Vec3::new(0.0, 14.0, 62.0);
        }
    }
}

fn setup_tnua(
    mut commands: Commands,
    query: Query<Entity, (With<NeedsTnua>, Without<TnuaController>)>,
    input_map: Res<PlayerInputMap>,
) {
    for entity in &query {
        commands.entity(entity).insert((
            TnuaController::default(),
            TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
            LockedAxes::ROTATION_LOCKED,
            InputManagerBundle::with_map(input_map.0.clone())
        )).remove::<NeedsTnua>();
    }
}


fn apply_controls(
    actions: Query<&ActionState<PlayerAction>>,
    mut query: Query<&mut TnuaController, With<Player>>,
) {
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;
    let mut jumping = false;

    for state in &actions {
        if state.pressed(&PlayerAction::Up) {
            direction -= Vec3::Z;
        }
        if state.pressed(&PlayerAction::Down) {
            direction += Vec3::Z;
        }
        if state.pressed(&PlayerAction::Left) {
            direction -= Vec3::X;
        }
        if state.pressed(&PlayerAction::Right) {
            direction += Vec3::X;
        }
        if state.pressed(&PlayerAction::Jump) {
            jumping = true;
        }
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 10.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.3,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if jumping {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 4.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }
}
