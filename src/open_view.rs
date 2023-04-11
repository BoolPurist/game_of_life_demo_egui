use std::{fmt::Display, num::ParseIntError, path::PathBuf, time::Duration};

use eframe::egui::{self, Ui};
use egui_file::FileDialog;
use getset::CopyGetters;
use thiserror::Error;

use crate::{
    constans::TICK_DURATION,
    grid::{text_load_error::TextLoadError, Grid, InvalidCharCell},
    CurrentView,
};

mod drawing;

#[derive(Clone, Copy, Debug)]
pub enum TimeUnit {
    Seconds(u32),
    MsSeconds(u32),
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seconds(secs) | Self::MsSeconds(secs) => write!(f, "{}", secs),
        }
    }
}

#[derive(Clone, CopyGetters)]
pub struct GatheredOpenViewData {
    dead_char_code: char,
    alive_char_code: char,
    #[getset(get_copy = "pub")]
    time_interval: TimeUnit,
    selected_time: SelectedTime,
    game: Grid,
    path: PathBuf,
}

impl GatheredOpenViewData {
    pub fn clone_game(&self) -> Grid {
        self.game.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedTime {
    Seconds,
    MsSeconds,
}
impl Default for SelectedTime {
    fn default() -> Self {
        Self::Seconds
    }
}
impl Display for SelectedTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seconds => write!(f, "Seconds"),
            Self::MsSeconds => write!(f, "Mili seconds"),
        }
    }
}

pub struct OpenView {
    open_file_dialog: Option<FileDialog>,
    dead_char_code: char,
    alive_char_code: char,
    selected_time: SelectedTime,
    dead_char_input: String,
    alive_char_input: String,
    time_interval: String,
    game_file_state: DataFileState,
}

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

impl From<TimeUnit> for Duration {
    fn from(value: TimeUnit) -> Self {
        match value {
            TimeUnit::Seconds(secs) => Duration::from_secs(secs as u64),
            TimeUnit::MsSeconds(ms_secs) => Duration::from_millis(ms_secs as u64),
        }
    }
}

impl Default for OpenView {
    fn default() -> Self {
        let dead_char_code = crate::constans::DEAD_CHAR;
        let alive_char_code = crate::constans::ALIVE_CHAR;

        Self {
            dead_char_code,
            alive_char_code,
            game_file_state: DataFileState::NotChoosen,
            open_file_dialog: None,
            selected_time: Default::default(),
            dead_char_input: String::from(dead_char_code),
            alive_char_input: String::from(alive_char_code),
            time_interval: TICK_DURATION.as_secs().to_string(),
        }
    }
}
impl OpenView {
    pub fn draw(&mut self, ctx: &egui::Context, ui: &mut Ui) -> Option<CurrentView> {
        drawing::draw_input_mask(self, ui, ctx)
    }

    pub fn new(gathered: GatheredOpenViewData) -> Self {
        let dead_char_code = gathered.dead_char_code;
        let alive_char_code = gathered.alive_char_code;
        Self {
            dead_char_code,
            alive_char_code,
            game_file_state: DataFileState::Loaded {
                game: gathered.game,
                path: gathered.path,
            },
            selected_time: gathered.selected_time,
            open_file_dialog: None,
            dead_char_input: dead_char_code.into(),
            alive_char_input: alive_char_code.into(),
            time_interval: gathered.time_interval.to_string(),
        }
    }
}

fn time_unit_from_selection(selected_kind: SelectedTime, amount: u32) -> TimeUnit {
    match selected_kind {
        SelectedTime::Seconds => TimeUnit::Seconds(amount),
        SelectedTime::MsSeconds => TimeUnit::MsSeconds(amount),
    }
}
