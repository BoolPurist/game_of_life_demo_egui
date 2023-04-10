use std::path::PathBuf;

use eframe::egui::{self, Ui};
use egui_file::FileDialog;

use crate::{
    grid::{text_load_error::TextLoadError, Grid, InvalidCharCell},
    CurrentView,
};

mod drawing;
#[derive(Clone)]
pub struct GatheredOpenViewData {
    dead_char_code: char,
    alive_char_code: char,
    game: Grid,
    path: PathBuf,
}

impl GatheredOpenViewData {
    pub fn clone_game(&self) -> Grid {
        self.game.clone()
    }
}

pub struct OpenView {
    open_file_dialog: Option<FileDialog>,
    dead_char_code: char,
    alive_char_code: char,
    dead_char_input: String,
    alive_char_input: String,
    game_file_state: DataFileState,
}

pub enum DataFileState {
    NotChoosen,
    Choosen {
        path: PathBuf,
    },
    Invalid {
        path: PathBuf,
        error: TextLoadError,
    },
    Loaded {
        path: PathBuf,
        game: Grid,
    },
    InvalidChars {
        path: PathBuf,
        error: InvalidCharCell,
    },
}

impl Default for DataFileState {
    fn default() -> Self {
        Self::NotChoosen
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
            dead_char_input: String::from(dead_char_code),
            alive_char_input: String::from(dead_char_code),
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
            open_file_dialog: None,
            dead_char_input: dead_char_code.into(),
            alive_char_input: alive_char_code.into(),
        }
    }
}
