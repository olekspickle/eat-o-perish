use std::{
    time::Duration,
};

use rand::Rng;


use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
};
use bevy_tnua::prelude::*;
use avian3d::prelude::*;
use blenvy::{
    AddToGameWorld, BlueprintInfo,
    HideUntilReady, SpawnBlueprint,
};


use rand::random;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Energy(u32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PelletEater;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CritterEater;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FoodPellet;


#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Critter;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Preditor;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Herbivore;


pub(super) fn plugin(app: &mut App) {
    app.register_type::<FoodPellet>();
    app.register_type::<Preditor>();
    app.register_type::<Herbivore>();
    app.register_type::<Critter>();
    app.add_systems(Update, (
        spawn_herbivores,
        spawn_preditors,
        herbivore_movement.run_if(on_timer(Duration::from_millis(500))),
        preditor_movement.run_if(on_timer(Duration::from_millis(500))),
        consume_energy.run_if(on_timer(Duration::from_secs(1))),
        eat_pellet,
        eat_critter,
    ));
}

fn spawn_herbivores(
    mut commands: Commands,
    query: Query<Entity, Added<Herbivore>>,
) {
    for entity in &query {
        commands.entity(entity).insert((
            BlueprintInfo::from_path("blueprints/Herbivore.glb"),
            SpawnBlueprint,
            HideUntilReady,
            AddToGameWorld,
            CollidingEntities::default(),
            Energy(10),
            PelletEater,
        ));
    }
}

fn spawn_preditors(
    mut commands: Commands,
    query: Query<Entity, Added<Preditor>>,
) {
    for entity in &query {
        commands.entity(entity).insert((
            BlueprintInfo::from_path("blueprints/Preditor.glb"),
            SpawnBlueprint,
            HideUntilReady,
            AddToGameWorld,
            CollidingEntities::default(),
            Energy(10),
            CritterEater,
        ));
    }
}


fn herbivore_movement(
    mut query: Query<&mut TnuaController, With<Herbivore>>,
) {
    for mut controller in &mut query {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-1.0..1.0);
        let z: f32 = rng.gen_range(-1.0..1.0);
        let direction = Vec3::new(x, 0.0, z);
        let jumping = random::<f32>() > 0.9;

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction.normalize_or_zero() * 10.0,
            float_height: 1.5,
            ..Default::default()
        });

        if jumping {
            controller.action(TnuaBuiltinJump {
                height: 4.0,
                ..Default::default()
            });
        }
    }
}

fn preditor_movement(
    mut query: Query<&mut TnuaController, With<Preditor>>,
) {
    for mut controller in &mut query {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-1.0..1.0);
        let z: f32 = rng.gen_range(-1.0..1.0);
        let direction = Vec3::new(x, 0.0, z);
        let jumping = random::<f32>() > 0.9;

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction.normalize_or_zero() * 10.0,
            float_height: 1.5,
            ..Default::default()
        });

        if jumping {
            controller.action(TnuaBuiltinJump {
                height: 4.0,
                ..Default::default()
            });
        }
    }
}

fn eat_pellet(
    mut commands: Commands,
    mut query: Query<(&CollidingEntities, Option<&mut Energy>), With<PelletEater>>,
    food_pellets: Query<Entity, With<FoodPellet>>,
) {
    for (colliding_entities, mut energy) in &mut query {
        for entity in &colliding_entities.0 {
            if food_pellets.contains(*entity) {
                commands.entity(*entity).despawn_recursive();
                if let Some(energy) = energy.as_mut() {
                    energy.0 += 1;
                }
            }
        }
    }
}

fn eat_critter(
    mut commands: Commands,
    mut query: Query<(&CollidingEntities, Option<&mut Energy>), With<CritterEater>>,
    critters: Query<Entity, With<Critter>>,
) {
    for (colliding_entities, mut energy) in &mut query {
        for entity in &colliding_entities.0 {
            if critters.contains(*entity) {
                commands.entity(*entity).despawn_recursive();
                if let Some(energy) = energy.as_mut() {
                    energy.0 += 10;
                }
            }
        }
    }
}

fn consume_energy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Energy)>,
) {
    for (entity, mut energy) in &mut query {
        if energy.0 == 0 {
            commands.entity(entity).despawn_recursive();
        } else {
            energy.0 -= 1;
        }
    }
}
