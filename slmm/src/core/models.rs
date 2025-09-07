use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game_driver::kind::GameDriverKind;

pub trait HasUuid {
    fn uuid(&self) -> Uuid;

    fn find_by_uuid(vec: Vec<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        vec.into_iter().next()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UuidFlaged {
    pub uuid: Uuid,
    pub is_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    pub uuid: Uuid,
    pub name: String,
    pub version: String,
    pub notes: String,
    pub install_path: PathBuf,
}
impl Mod {
    pub fn new(name: String, version: String, notes: String, install_path: PathBuf) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            version,
            notes,
            install_path,
        }
    }
}

impl HasUuid for Mod {
    fn uuid(&self) -> Uuid {
        self.uuid
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub uuid: Uuid,
    pub name: String,
    pub mod_ids: Vec<UuidFlaged>,
}
impl Profile {
    pub fn new(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            mod_ids: Vec::new(),
        }
    }
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: "Default".to_owned(),
            mod_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub artifact_install_path: PathBuf,
    pub active_profile_id: Uuid,
    pub available_mods: Vec<Mod>,
    pub available_profiles: Vec<Profile>,
    pub game_driver: GameDriverKind,
}

impl GameState {
    pub fn new(artifact_install_path: PathBuf, game_driver: GameDriverKind) -> Self {
        let new_profile = Profile::default();
        Self {
            artifact_install_path,
            active_profile_id: new_profile.uuid,
            available_mods: Vec::new(),
            available_profiles: vec![new_profile],
            game_driver,
        }
    }
}
