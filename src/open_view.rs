use std::path::PathBuf;

use eframe::egui::{self, Ui};
use egui_file::FileDialog;

use crate::CurrentView;

mod drawing;
pub struct OpenView {
    path: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
    dead_char_code: char,
    alive_char_code: char,
}

impl Default for OpenView {
    fn default() -> Self {
        Self {
            path: None,
            dead_char_code: crate::constans::DEAD_CHAR,
            alive_char_code: crate::constans::ALIVE_CHAR,
            open_file_dialog: None,
        }
    }
}
impl OpenView {
    pub fn draw(&mut self, ctx: &egui::Context, ui: &mut Ui) -> Option<CurrentView> {
        drawing::draw_input_mask(self, ui, ctx)
    }
}
