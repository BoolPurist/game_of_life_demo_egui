use std::path::PathBuf;

use getset::{CopyGetters, Getters};

use crate::grid::Grid;

use super::{SelectedTime, TimeUnit};

#[derive(Clone, CopyGetters, Getters)]
pub struct GatheredOpenViewData {
    pub dead_char_code: char,
    pub alive_char_code: char,
    pub selected_time: SelectedTime,
    pub time_interval: TimeUnit,
    pub game: Grid,
    pub path: PathBuf,
}

impl GatheredOpenViewData {
    pub fn clone_game(&self) -> Grid {
        self.game.clone()
    }
}
