use std::time::Instant;

use crate::data;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Component)]
pub struct TeamMember(pub Option<data::Id>);

#[derive(Resource)]
pub struct Game {
    pub wave_set_definition_id: data::Id,
    pub wave: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Menu,
    InGame,
}

#[derive(Resource)]
pub struct WaveStart(pub Instant);

pub fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in &query {
        if !health.0.is_positive() {
            commands.entity(entity).despawn()
        }
    }
}

fn run() {
    App::new().add_plugins(DefaultPlugins).run();
}
