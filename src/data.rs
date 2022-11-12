use serde::{Deserialize, Serialize};
use std::time::Duration;

pub type Id = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityDefinition {
    pub id: Id,
    pub name: String,
    pub max_health: u16,
    pub speed: f64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct SpawnDefinition {
    pub entity_definition_id: Id,
    pub team_id: Id,
    pub delay: Duration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaveDefinition {
    pub id: Id,
    pub spawn_definitions: Vec<SpawnDefinition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunDefinition {
    pub id: Id,
    pub wave_definition_ids: Vec<Id>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub entity_definitions: Vec<EntityDefinition>,
    pub wave_definitions: Vec<WaveDefinition>,
    pub run_definitions: Vec<RunDefinition>,
}
