use crate::core::models::UuidFlaged;
use crate::core::models::{GameState, Mod, Profile};
use crate::core::storage::Storage;
use crate::error::Result;
use crate::game_driver::kind::GameDriverKind;
use crate::game_driver::traits::GameDriver;
use fs_extra::dir::{CopyOptions, copy, remove};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug)]
pub struct ModManager {
    storage: Storage,
    current_state: GameState,
}

impl ModManager {
    pub fn new(
        artifact_install_path: PathBuf,
        storage: Storage,
        game_driver: GameDriverKind,
    ) -> Result<Self> {
        Ok(Self {
            storage,
            current_state: GameState::new(artifact_install_path, game_driver),
        })
    }
    pub fn deploy(&self) -> Result<()> {
        info!("trying to deploy mod manager...");

        let profile_id = self.current_state.active_profile_id;
        let profile_index = self.get_profile_index(profile_id)?;
        let profile = &self.current_state.available_profiles[profile_index];

        info!("trying to deploy profile: {:#?}...", profile);

        let mods: Vec<&Mod> = profile
            .mod_ids
            .iter()
            .filter_map(|uuid_flaged| {
                self.current_state
                    .available_mods
                    .iter()
                    .find(|m| uuid_flaged.is_enabled && m.uuid == uuid_flaged.uuid)
            })
            .collect();

        self.current_state.game_driver.deploy(mods)
    }

    pub fn load(storage: Storage) -> Result<Self> {
        let current_state = storage.load_game_state()?;
        Ok(Self {
            storage,
            current_state,
        })
    }

    pub fn install_mod(&mut self, name: String, version: String, mod_path: &Path) -> Result<Uuid> {
        info!(
            %name,
            %version,
            ?mod_path,
            "trying to install mod..."
        );

        let uuid = Uuid::new_v4();
        let install_path = self
            .current_state
            .artifact_install_path
            .join(uuid.to_string());

        let mut options = CopyOptions::new();
        options.copy_inside = true;
        remove(&install_path)?;
        copy(mod_path, &install_path, &options)?;

        self.current_state.available_mods.push(Mod {
            uuid,
            name,
            version,
            notes: String::new(),
            install_path,
        });

        Ok(uuid)
    }

    pub fn uninstall_mod(&mut self, mod_id: &Uuid) -> Result<()> {
        info!(
            %mod_id,
            "trying to uninstall mod..."
        );

        if let Some(pos) = &self
            .current_state
            .available_mods
            .iter()
            .position(|m| &m.uuid == mod_id)
        {
            let u_mod = &self.current_state.available_mods.remove(*pos);
            fs::remove_dir_all(&u_mod.install_path)?;
        } else {
            warn!(
                %mod_id,
                "mod uuid not found"
            );
        };
        Ok(())
    }

    pub fn create_profile(&mut self, name: &str) -> Profile {
        Profile::new(name.to_owned())
    }

    pub fn get_profile_index(&self, profile_id: Uuid) -> Result<usize> {
        match self
            .current_state
            .available_profiles
            .iter()
            .position(|m| m.uuid == profile_id)
        {
            Some(profile) => {
                info!("Found profile index: {:#?}", profile);
                Ok(profile)
            }
            None => {
                error!("Could not find profile with uuid: {:?}", profile_id);
                Err(crate::error::ModManagerError::ProfileNotFound(profile_id))
            }
        }
    }

    pub fn activate_profile(&mut self, profile_id: Uuid) -> Result<()> {
        let _profile_index = self.get_profile_index(profile_id)?;
        self.current_state.active_profile_id = profile_id;
        self.sync_mods(profile_id)?;

        Ok(())
    }

    pub fn get_mod(&self, mod_id: Uuid) -> Result<&Mod> {
        self.current_state
            .available_mods
            .iter()
            .find(|m| m.uuid == mod_id)
            .ok_or(crate::error::ModManagerError::ModNotFound(mod_id))
    }

    pub fn get_mod_mut(&mut self, mod_id: Uuid) -> Result<&mut Mod> {
        self.current_state
            .available_mods
            .iter_mut()
            .find(|m| m.uuid == mod_id)
            .ok_or(crate::error::ModManagerError::ModNotFound(mod_id))
    }

    pub fn get_profile(&self, profile_id: Uuid) -> Result<&Profile> {
        self.current_state
            .available_profiles
            .iter()
            .find(|p| p.uuid == profile_id)
            .ok_or(crate::error::ModManagerError::ProfileNotFound(profile_id))
    }

    pub fn get_profile_mut(&mut self, profile_id: Uuid) -> Result<&mut Profile> {
        self.current_state
            .available_profiles
            .iter_mut()
            .find(|p| p.uuid == profile_id)
            .ok_or(crate::error::ModManagerError::ProfileNotFound(profile_id))
    }

    pub fn sync_mods(&mut self, profile_id: Uuid) -> Result<()> {
        let available_mods: Vec<Uuid> = self
            .current_state
            .available_mods
            .iter()
            .map(|m| m.uuid)
            .collect();

        let profile = self.get_profile_mut(profile_id)?;

        for uuid in available_mods {
            if !profile.mod_ids.iter().any(|x| x.uuid == uuid) {
                profile.mod_ids.push(UuidFlaged {
                    uuid,
                    is_enabled: false,
                });
            }
        }
        Ok(())
    }

    pub fn enable_mod(&mut self, mod_id: Uuid) -> Result<()> {
        let profile = self.get_profile_mut(self.current_state.active_profile_id)?;
        profile.mod_ids.iter_mut().for_each(|m| {
            if m.uuid == mod_id {
                if !m.is_enabled {
                    m.is_enabled = true;
                    info!("mod {:?} enabled in profile {:?}", mod_id, profile.uuid)
                } else {
                    warn!(
                        "mod {:?} is already enabled in profile {:?}",
                        mod_id, profile.uuid
                    )
                }
            }
        });
        Ok(())
    }

    pub fn disable_mod(&mut self, mod_id: Uuid) -> Result<()> {
        let profile = self.get_profile_mut(self.current_state.active_profile_id)?;
        profile.mod_ids.iter_mut().for_each(|m| {
            if m.uuid == mod_id {
                if m.is_enabled {
                    m.is_enabled = false;
                    info!("mod {:?} disabled in profile {:?}", mod_id, profile.uuid)
                } else {
                    warn!(
                        "mod {:?} is already disabled in profile {:?}",
                        mod_id, profile.uuid
                    )
                }
            }
        });
        Ok(())
    }

    pub fn load_game_state(file_config: PathBuf) -> Result<Self> {
        let storage = Storage::new(file_config);
        let current_state = storage.load_game_state()?;
        let mut result = Self {
            storage,
            current_state,
        };
        result.sync_mods(result.current_state.active_profile_id)?;
        Ok(result)
    }
    pub fn save_game_state(&self) -> Result<()> {
        self.storage.save_game_state(&self.current_state)
    }
}
