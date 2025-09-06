use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    pub uuid: Uuid,
    pub name: String,
    pub version: String,
    pub notes: String,
    pub install_path: PathBuf,
    pub is_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub uuid: Uuid,
    pub name: String,
    pub mod_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub game_install_path: PathBuf,
    pub artifact_install_path: PathBuf,
    pub active_profile_id: Option<Uuid>,
    pub available_mods: Vec<Mod>,
    pub available_profiles: Vec<Profile>,
}
