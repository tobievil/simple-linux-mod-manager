use crate::core::models::Mod;
use crate::error::Result;

pub trait GameDriver {
    fn deploy(&self, mods: Vec<&Mod>) -> Result<()>;
}
