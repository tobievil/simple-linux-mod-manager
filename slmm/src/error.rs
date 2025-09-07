use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum ModManagerError {
    #[error("IO operation failed: {0}")]
    IO(#[from] std::io::Error),
    #[error("IO operation failed: {0}")]
    FSExtra(#[from] fs_extra::error::Error),
    #[error("Could not find Profile by uuid: {0}")]
    ProfileNotFound(Uuid),
    #[error("Could not find Mod by uuid: {0}")]
    ModNotFound(Uuid),
}

pub type Result<T> = std::result::Result<T, ModManagerError>;
