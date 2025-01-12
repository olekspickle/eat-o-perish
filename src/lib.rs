mod asset_tracking;
pub mod audio;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screens;
mod theme;

use std::time::Duration;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};

use avian3d::prelude::*;
use bevy_spatial::{AutomaticUpdate, SpatialStructure, TransformMode};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;
use blenvy::BlenvyPlugin;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Spawn the main camera.
        // TODO: Not sure what this does?..
        app.add_systems(Startup, spawn_ui_camera);

        // Add Bevy plugins.
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bevy New 3D".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
            BlenvyPlugin::default(),
            PhysicsPlugins::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
            // Camera
            LookTransformPlugin,
            OrbitCameraPlugin::default(),
            // Herbivores
            AutomaticUpdate::<crate::game::critters::FoodPellet>::new()
                .with_spatial_ds(SpatialStructure::KDTree3)
                .with_frequency(Duration::from_secs_f32(0.5))
                .with_transform(TransformMode::GlobalTransform),
            AutomaticUpdate::<crate::game::critters::Herbivore>::new()
                .with_spatial_ds(SpatialStructure::KDTree3)
                .with_frequency(Duration::from_secs_f32(0.5))
                .with_transform(TransformMode::GlobalTransform),
        ));

        app.register_type::<bevy::text::TextEntity>();

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            game::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UiCamera"),
        Camera {
            order: 2,
            ..Default::default()
        },
        Camera2d,
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}
