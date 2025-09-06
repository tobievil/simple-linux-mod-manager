#[derive(thiserror::Error, Debug)]
pub enum ModManagerError {
    #[error("IO operation failed: {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ModManagerError>;
