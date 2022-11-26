use crate::data::{self, Id};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::time::{Duration, Instant};

/// Units

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Team(pub Id);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Move {
    velocity: f32,
}

/// Wave handling

#[derive(Resource)]
pub struct Game {
    pub wave_set_definition_id: Id,
    pub wave: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Menu,
    InGame,
}

pub struct NewWaveEvent;

#[derive(Resource)]
pub struct WaveStart(pub Instant);

pub fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in &query {
        if health.0 == 0 {
            commands.entity(entity).despawn()
        }
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Unit,
        Team(0),
        Health(100),
        Move { velocity: 1.0 },
        SpriteBundle {
            texture: asset_server.load("player.png"),
            ..Default::default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn keyboard_input(mut query: Query<&mut Transform, With<Camera2d>>, keys: Res<Input<KeyCode>>) {
    let mut camera_transform = query.single_mut();
    if keys.pressed(KeyCode::Right) {
        camera_transform.translation.x += 1.;
    } else if keys.pressed(KeyCode::Left) {
        camera_transform.translation.x -= 1.;
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_fixed_timestep(Duration::from_millis(100), "core_loop")
        .add_fixed_timestep_system("core_loop", 0, keyboard_input)
        .run();
}
