use eframe::{
    egui::{self, Ui, WidgetText},
    epaint::Color32,
};

use crate::constans::GRID_SPACEING;

pub fn computed_value(ui: &mut Ui, text: impl Into<WidgetText>) {
    ui.label(text.into().strong());
}
pub fn computed_with_color(ui: &mut Ui, text: impl Into<WidgetText>, color: Color32) {
    ui.label(text.into().color(color).strong());
}

pub fn draw_grid(ui: &mut Ui, grid_name: &str, on_draw: impl FnOnce(&mut Ui)) {
    egui::Grid::new(grid_name)
        .num_columns(2)
        .spacing(GRID_SPACEING)
        .striped(true)
        .show(ui, on_draw);
}
