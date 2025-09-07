use crate::core::models::Mod;
use crate::error::Result;
use crate::game_driver::traits::GameDriver;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericGameDriver {
    pub game_install_path: PathBuf,
}

impl GenericGameDriver {
    pub fn new(game_install_path: PathBuf) -> Self {
        Self { game_install_path }
    }
}

// TODO: add trait abstraction to backend of copying/projecting mod files.
impl GameDriver for GenericGameDriver {
    fn deploy(&self, mods: Vec<&Mod>) -> Result<()> {
        info!("trying to deploy mods: {:#?}", mods);
        // let command = "mergerfs";
        let command = "echo";
        let mut args = vec![
            "-o",
            "defaults,use_ino,category.create=ff,category.action=epff",
        ];
        let input_paths = mods
            .iter()
            .map(|m| m.install_path.to_str().unwrap())
            .collect::<Vec<&str>>()
            .join(":");

        args.push(&input_paths);

        let output = Command::new(command)
            .args(args)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Output: {}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", stderr);
        }

        Ok(())
    }
}
