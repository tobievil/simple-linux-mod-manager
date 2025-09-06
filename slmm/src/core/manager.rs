use std::fs;
use std::path::Path;

use std::process::Command;
use uuid::Uuid;

use crate::core::models::{GameState, Mod, Profile};
use crate::core::storage::Storage;
use crate::error::Result;
use crate::utils::{copy_dir_all, create_empty_dir};

pub struct ModManager {
    storage: Storage,
    current_state: GameState,
}

impl ModManager {
    pub fn new(storage: Storage) -> Result<Self> {
        let current_state = storage.load_game_state()?;
        Ok(Self {
            storage,
            current_state,
        })
    }

    pub fn install_mod(&mut self, name: String, version: String, mod_path: &Path) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        let install_path = self
            .current_state
            .artifact_install_path
            .join(uuid.to_string());

        create_empty_dir(&install_path)?;
        copy_dir_all(mod_path, &install_path)?;

        self.current_state.available_mods.push(Mod {
            uuid,
            name,
            version,
            notes: String::new(),
            install_path,
            is_enabled: false,
        });

        Ok(uuid)
    }

    pub fn uninstall_mod(&mut self, mod_id: &Uuid) -> Result<()> {
        if let Some(pos) = &self
            .current_state
            .available_mods
            .iter()
            .position(|m| &m.uuid == mod_id)
        {
            let u_mod = &self.current_state.available_mods.remove(*pos);
            fs::remove_dir_all(&u_mod.install_path)?;
        } else {
            println!("UUID: {} not found when trying to uninstall mod", &mod_id)
        };
        Ok(())
    }

    pub fn create_profile(&mut self, name: &str) -> Result<Profile> {
        todo!()
    }

    pub fn switch_profile(&mut self, profile_id: &Uuid) -> Result<()> {
        todo!()
    }

    pub fn enable_mod(&mut self, mod_id: &Uuid) -> Result<()> {
        todo!()
    }

    pub fn disable_mod(&mut self, mod_id: &Uuid) -> Result<()> {
        todo!()
    }

    pub fn deploy(&self) -> Result<()> {
        let profile_id = self
            .current_state
            .active_profile_id
            .expect("no profile is active");

        // TODO:
        // factor out find funcs
        let profile = &self
            .current_state
            .available_profiles
            .iter()
            .find(|m| m.uuid == profile_id)
            .unwrap_or_else(|| {
                panic!(
                    "UUID: {} not found when trying to deploy profile",
                    &profile_id
                )
            });
        let ordered_mods: Vec<&Mod> = profile
            .mod_ids
            .iter()
            .filter_map(|&uuid| {
                self.current_state
                    .available_mods
                    .iter()
                    .find(|s| s.uuid == uuid)
            })
            .collect();

        let command = "mergerfs";
        let mut args = vec![
            "-o",
            "defaults,use_ino,category.create=ff,category.action=epff",
        ];
        let input_paths = ordered_mods
            .iter()
            .map(|m| m.install_path.to_str().unwrap())
            .collect::<Vec<&str>>()
            .join(":");

        args.push(&input_paths);

        // TODO:
        // add overwrite dir and output dir

        let output = Command::new(command)
            .args(args)
            .output()
            .expect("Failed to execute command");

        // Check if the command was successful
        if output.status.success() {
            // Convert the output to a String and print it
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Output: {}", stdout);
        } else {
            // Convert the error output to a String and print it
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", stderr);
        }

        Ok(())
    }

    pub fn detect_game(&mut self) -> Result<()> {
        todo!()
    }
}
