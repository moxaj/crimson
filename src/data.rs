use bevy::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

pub type Id = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitDefinition {
    pub id: Id,
    pub name: String,
    pub max_health: u16,
    pub speed: f64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct SpawnDefinition {
    pub unit_definition_id: Id,
    pub team_id: Id,
    pub delay: Duration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaveDefinition {
    pub id: Id,
    pub spawn_definitions: Vec<SpawnDefinition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaveSetDefinition {
    pub id: Id,
    pub wave_definition_ids: Vec<Id>,
}

#[derive(Serialize, Deserialize, Resource, Debug, Clone)]
pub struct Data {
    pub unit_definitions: HashMap<Id, UnitDefinition>,
    pub wave_definitions: HashMap<Id, WaveDefinition>,
    pub wave_set_definitions: HashMap<Id, WaveSetDefinition>,
}

pub static DATA: Lazy<Data> =
    Lazy::new(|| serde_json::from_str::<Data>(include_str!("data.json")).unwrap());
