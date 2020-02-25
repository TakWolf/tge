use crate::error::{GameError, GameResult};
use std::path::Path;

pub struct Filesystem {}

impl Filesystem {

    pub(crate) fn new(_: FilesystemConfig) -> GameResult<Self> {
        Ok(Self {})
    }

    pub fn read<P: AsRef<Path>>(&self, path: P) -> GameResult<Vec<u8>> {
        std::fs::read(path).map_err(|error| GameError::IoError(Box::new(error)))
    }

    pub fn read_to_string<P: AsRef<Path>>(&self, path: P) -> GameResult<String> {
        std::fs::read_to_string(path).map_err(|error| GameError::IoError(Box::new(error)))
    }

}

#[derive(Debug, Clone)]
pub struct FilesystemConfig {}

impl FilesystemConfig {

    pub fn new() -> Self {
        Self {}
    }

}
