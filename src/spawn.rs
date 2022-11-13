use bevy::prelude::*;

use crate::data;
use crate::game;

#[derive(Component)]
pub struct SpawnTimer {
    spawn_definition: data::SpawnDefinition,
    timer: Timer,
}

pub fn clear_spawn_timers(mut commands: Commands, query: Query<Entity, With<SpawnTimer>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn setup_spawn_timers(mut commands: Commands, data: Res<data::Data>, game: Res<game::Game>) {
    let wave_set_definition = &data.wave_set_definitions[&game.wave_set_definition_id];
    let wave_id = wave_set_definition.wave_definition_ids[game.wave];
    for spawn_definition in &data.wave_definitions[&wave_id].spawn_definitions {
        commands.spawn(SpawnTimer {
            spawn_definition: *spawn_definition,
            timer: Timer::new(spawn_definition.delay, TimerMode::Once),
        });
    }
}

pub fn spawn_entities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpawnTimer)>,
    time: Res<Time>,
    data: Res<data::Data>,
) {
    for (spawn_timer_entity, mut spawn_timer) in query.iter_mut() {
        if spawn_timer.timer.tick(time.delta()).just_finished() {
            let spawn_definition = spawn_timer.spawn_definition;
            let unit_definition = &data.unit_definitions[&spawn_definition.unit_definition_id];
            commands.spawn((
                game::Health(unit_definition.max_health.into()),
                game::TeamMember(Some(spawn_definition.team_id)),
            ));
            commands.entity(spawn_timer_entity).despawn();
        }
    }
}
