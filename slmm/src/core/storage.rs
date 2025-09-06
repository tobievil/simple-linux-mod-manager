use crate::core::models::GameState;
use crate::error::Result;
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    file_config: PathBuf,
}

impl Storage {
    pub fn load_game_state(&self) -> Result<GameState> {
        let json_string = fs::read_to_string(&self.file_config)?;
        let game_state: GameState = serde_json::from_str(&json_string).unwrap();
        Ok(game_state)
    }
    pub fn save_game_state(&self, state: &GameState) -> Result<()> {
        let json_string = serde_json::to_string(state).unwrap();
        fs::write(&self.file_config, json_string)?;
        Ok(())
    }
}
