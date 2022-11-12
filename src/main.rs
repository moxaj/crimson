mod data;

use std::time::{Duration, Instant};

use bevy::prelude::*;
use data::{Data, Id, RunDefinition, SpawnDefinition};

/*

Vampire Survivors clone

*/

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Component)]
struct Velocity {
    x: f64,
    y: f64,
}

#[derive(Component)]
struct TeamMember(Option<u16>);

struct TeamDynamics;

#[derive(Component)]
enum AI {
    AttackNearest { target: Option<Entity> },
}

struct Game {
    run_definition: RunDefinition,
    wave: usize,
}

struct RunStart(Instant);

struct WaveStart(Instant);

#[derive(Component)]
struct SpawnTimer {
    spawn_definition: SpawnDefinition,
    timer: Timer,
}

/// Systems

fn setup_spawn_timers(mut commands: Commands, game: Res<Game>, data: Res<Data>) {
    let wave_id = game.run_definition.wave_definition_ids[game.wave];
    for spawn_definition in &data
        .wave_definitions
        .iter()
        .find(|wave_definition| wave_definition.id == wave_id)
        .unwrap()
        .spawn_definitions
    {
        commands.spawn().insert(SpawnTimer {
            spawn_definition: *spawn_definition,
            timer: Timer::new(spawn_definition.delay, false),
        });
    }
}

fn spawn_entities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpawnTimer)>,
    time: Res<Time>,
    data: Res<Data>,
) {
    for (spawn_timer_entity, mut spawn_timer) in query.iter_mut() {
        if spawn_timer.timer.tick(time.delta()).just_finished() {
            let spawn_definition = spawn_timer.spawn_definition;
            let entity_definition = data
                .entity_definitions
                .iter()
                .find(|entity| entity.id == spawn_definition.entity_definition_id)
                .unwrap();
            commands
                .spawn()
                .insert(Health(entity_definition.max_health.into()));
            commands.entity(spawn_timer_entity).despawn();
        }
    }
}

fn kill_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in &query {
        if !health.0.is_positive() {
            commands.entity(entity).despawn()
        }
    }
}

fn main() {
    // App::new().add_plugins(DefaultPlugins).run();
    println!(
        "{:?}",
        serde_json::from_str::<Data>(include_str!("data.json"))
    )
}
