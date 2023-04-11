use eframe::egui::{self, Ui};
use egui_file::FileDialog;

use crate::{constans::TICK_DURATION, CurrentView};

mod data_file_state;
mod drawing;
mod gathered_open_view_data;
mod time;
mod validation_error;

pub use data_file_state::DataFileState;
pub use gathered_open_view_data::GatheredOpenViewData;
pub use time::{SelectedTime, TimeUnit};
pub use validation_error::ValidationError;

pub struct OpenView {
    open_file_dialog: Option<FileDialog>,
    dead_char_code: char,
    alive_char_code: char,
    dead_char_input: String,
    alive_char_input: String,
    selected_time: SelectedTime,
    time_interval: String,
    game_file_state: DataFileState,
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
        let path = gathered.path;
        Self {
            dead_char_code,
            alive_char_code,
            selected_time: gathered.selected_time,
            game_file_state: DataFileState::Choosen { path },
            open_file_dialog: None,
            dead_char_input: dead_char_code.into(),
            alive_char_input: alive_char_code.into(),
            time_interval: gathered.time_interval.to_string(),
        }
    }
}
