use crate::core::models::Mod;
use crate::error::Result;
use crate::game_driver::generic::GenericGameDriver;
use crate::game_driver::traits::GameDriver;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum GameDriverKind {
    Generic(GenericGameDriver),
}

impl GameDriverKind {
    pub fn detect_game(&mut self) -> Result<()> {
        todo!()
    }
}

impl GameDriver for GameDriverKind {
    fn deploy(&self, mods: Vec<&Mod>) -> Result<()> {
        match self {
            GameDriverKind::Generic(o) => o.deploy(mods),
        }
    }
}
