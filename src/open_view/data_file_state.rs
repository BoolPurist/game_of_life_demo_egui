use std::path::PathBuf;

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
}

impl Default for DataFileState {
    fn default() -> Self {
        Self::NotChoosen
    }
}
impl From<(PathBuf, ValidationError)> for DataFileState {
    fn from(value: (PathBuf, ValidationError)) -> Self {
        DataFileState::Invalid {
            path: value.0,
            error: value.1,
        }
    }
}
