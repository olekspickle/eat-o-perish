//! Spawn the main level.

use rand::Rng;

use bevy::prelude::*;
use blenvy::*;

use crate::game::critters::{FoodPellet, Herbivore, Preditor};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (
        spawn_food_pellet,
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
