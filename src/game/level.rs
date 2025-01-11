use std::time::Duration;

use rand::prelude::*;

use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
};
use blenvy::*;

use crate::game::critters::{FoodPellet, Herbivore, Preditor};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FloorPlate;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<FloorPlate>();
    app.add_systems(Update, (
        spawn_food_pellet,
        food_pellet_rain.run_if(on_timer(Duration::from_millis(10))),
    ));
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World) {
    world.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));

    let mut rng = rand::thread_rng();
    for _ in 0..30 {
        let location = Vec3::new(rng.gen_range(-80.0..80.0), 2.0, rng.gen_range(-80.0..80.0));
        world.spawn((
            Herbivore,
            Transform::from_translation(location),
        ));
    }
    for _ in 0..3 {
        let location = Vec3::new(rng.gen_range(-80.0..80.0), 2.0, rng.gen_range(-80.0..80.0));
        world.spawn((
            Preditor,
            Transform::from_translation(location),
        ));
    }
    for _ in 0..200 {
        let location = Vec3::new(rng.gen_range(-80.0..80.0), 2.0, rng.gen_range(-80.0..80.0));
        world.spawn((
            FoodPellet,
            Transform::from_translation(location),
        ));
    }
}

fn spawn_food_pellet(
    mut commands: Commands,
    query: Query<Entity, Added<FoodPellet>>,
) {
    for entity in &query {
        commands.entity(entity).insert((
            BlueprintInfo::from_path("blueprints/FoodPellet.glb"),
            SpawnBlueprint,
            HideUntilReady,
            AddToGameWorld,
        ));
    }
}

fn food_pellet_rain(
    mut commands: Commands,
    existing_pellets: Query<Entity, With<FoodPellet>>,
    floor_plates: Query<(Entity, &GlobalTransform), With<FloorPlate>>,
    children: Query<&Children>,
    aabs: Query<&bevy::render::primitives::Aabb>,
) {
    if existing_pellets.iter().count() > 1000 {
        return
    }
    let mut rng = rand::thread_rng();
    if let Some((entity, transform)) = floor_plates.iter().choose(&mut rng) {
        for child in children.iter_descendants(entity) {
            if let Ok(aab) = aabs.get(child) {
                let location = transform.transform_point(Vec3::new(rng.gen_range(-aab.half_extents.x..aab.half_extents.x), 20.0, rng.gen_range(-aab.half_extents.z..aab.half_extents.z)) + Vec3::from(aab.center));
                commands.spawn((
                    FoodPellet,
                    Transform::from_translation(location),
                ));
                return
            }
        }
    }
}
