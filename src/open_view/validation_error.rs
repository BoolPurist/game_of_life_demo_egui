use std::num::ParseIntError;

use thiserror::Error;

use crate::grid::{text_load_error::TextLoadError, InvalidCharCell};
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("{0}")]
    FailureInLoad(#[from] TextLoadError),
    #[error("Time must be a positive number")]
    NotNumberForTime,
    #[error("{0}")]
    InvalidChars(#[from] InvalidCharCell),
}

impl From<ParseIntError> for ValidationError {
    fn from(_value: ParseIntError) -> Self {
        Self::NotNumberForTime
    }
}
