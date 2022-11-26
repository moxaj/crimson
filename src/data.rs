use bevy::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

pub type Id = u64;

trait Index {
    fn id(&self) -> Id;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitDefinition {
    pub id: Id,
    pub name: String,
    pub max_health: u16,
    pub speed: f64,
}

impl Index for UnitDefinition {
    fn id(&self) -> Id {
        self.id
    }
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

impl Index for WaveDefinition {
    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaveSetDefinition {
    pub id: Id,
    pub wave_definition_ids: Vec<Id>,
}

impl Index for WaveSetDefinition {
    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Serialize, Deserialize, Resource, Debug, Clone)]
struct RawData {
    unit_definitions: Vec<UnitDefinition>,
    wave_definitions: Vec<WaveDefinition>,
    wave_set_definitions: Vec<WaveSetDefinition>,
}

#[derive(Serialize, Deserialize, Resource, Debug, Clone)]
pub struct Data {
    pub unit_definitions: HashMap<Id, UnitDefinition>,
    pub wave_definitions: HashMap<Id, WaveDefinition>,
    pub wave_set_definitions: HashMap<Id, WaveSetDefinition>,
}

impl From<RawData> for Data {
    fn from(raw_data: RawData) -> Self {
        fn indexed<T: Index>(vec: Vec<T>) -> HashMap<Id, T> {
            vec.into_iter().map(|item| (item.id(), item)).collect()
        }

        Data {
            unit_definitions: indexed(raw_data.unit_definitions),
            wave_definitions: indexed(raw_data.wave_definitions),
            wave_set_definitions: indexed(raw_data.wave_set_definitions),
        }
    }
}

pub static DATA: Lazy<Data> = Lazy::new(|| {
    serde_json::from_str::<RawData>(include_str!("data.json"))
        .unwrap()
        .into()
});
