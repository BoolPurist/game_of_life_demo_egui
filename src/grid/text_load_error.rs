use std::{io, path::PathBuf};

use thiserror::Error;
#[derive(Debug, Error)]
pub enum TextLoadError {
    #[error("File could not be loaded: {0}")]
    IoError(#[source] io::Error),
    #[error("There is no file at the path")]
    NoFileFound(PathBuf),
    #[error("Char {0} is not a dead or an alive cell")]
    NotValidCellChar(char),
    #[error("Row at {0} has a width which not equal to the others")]
    UnEqualWidth(usize),
}
