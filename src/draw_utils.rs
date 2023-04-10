use eframe::{
    egui::{self, Button, RichText, Ui},
    epaint::Color32,
};

use crate::constans::{FONT_SIZE, GRID_SPACEING};

pub fn computed_value(ui: &mut Ui, text: impl Into<String>) {
    ui.label(create_rich_text(text));
}
pub fn computed_with_color(ui: &mut Ui, text: impl Into<String>, color: Color32) {
    ui.label(create_rich_text(text).color(color));
}

pub fn button(text: impl Into<String>) -> Button {
    Button::new(create_rich_text(text))
}
pub fn button_with_color(text: impl Into<String>, color: Color32) -> Button {
    Button::new(create_rich_text(text).color(color))
}

pub fn create_rich_text(text: impl Into<String>) -> RichText {
    RichText::new(text).size(FONT_SIZE).strong()
}

pub fn draw_grid(ui: &mut Ui, grid_name: &str, on_draw: impl FnOnce(&mut Ui)) {
    egui::Grid::new(grid_name)
        .num_columns(2)
        .spacing(GRID_SPACEING)
        .striped(true)
        .show(ui, on_draw);
}
