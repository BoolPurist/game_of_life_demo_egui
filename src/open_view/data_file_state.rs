use std::path::PathBuf;

use crate::grid::Grid;

use super::ValidationError;

pub enum DataFileState {
    NotChoosen,
    Choosen {
        path: PathBuf,
    },
    Invalid {
        path: PathBuf,
        error: ValidationError,
    },
    Loaded {
        path: PathBuf,
        game: Grid,
    },
}

impl Default for DataFileState {
    fn default() -> Self {
        Self::NotChoosen
    }
}
