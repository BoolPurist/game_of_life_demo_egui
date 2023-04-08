use super::OpenView;
use crate::{constans::*, draw_utils};
use crate::{game_view::GameView, CurrentView};
use eframe::egui::{self, Button, Ui};
use eframe::epaint::Color32;
use egui_file::FileDialog;

const BTN_TEXT_LOAD: &str = "Load";
pub fn draw_input_mask(
    state: &mut OpenView,
    ui: &mut Ui,
    ctx: &egui::Context,
) -> Option<CurrentView> {
    draw_path_and_chars_for_text(state, ui);

    draw_buttons(state, ui, ctx)
}

fn draw_buttons(state: &mut OpenView, ui: &mut Ui, ctx: &egui::Context) -> Option<CurrentView> {
    let mut clicked_load = false;

    ui.horizontal(|ui| {
        if ui.button("Choose").clicked() {
            let mut dialog = FileDialog::open_file(state.path.clone());
            dialog.open();
            state.open_file_dialog = Some(dialog);
        }
        if let Some(dialog) = &mut state.open_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    state.path = Some(file);
                }
            }
        }

        if state.path.is_some() {
            clicked_load = ui.button(BTN_TEXT_LOAD).clicked();
        } else {
            ui.add_enabled(false, Button::new(BTN_TEXT_LOAD));
        }
    });

    if let Some(ref path) = state.path {
        if clicked_load {
            return Some(CurrentView::Game(GameView::new(
                path.clone(),
                TICK_DURATION,
            )));
        }
    }

    None
}

fn draw_path_and_chars_for_text(state: &mut OpenView, ui: &mut Ui) {
    draw_utils::draw_grid(ui, "Input grid", |ui| {
        draw_path_line(state, ui);

        ui.label("Char alive cell:");
        draw_utils::computed_value(ui, state.alive_char_code.to_string());
        ui.end_row();

        ui.label("Char dead cell:");
        draw_utils::computed_value(ui, state.dead_char_code.to_string());
        ui.end_row();
    });
}

fn draw_path_line(state: &mut OpenView, ui: &mut Ui) {
    draw_utils::computed_value(ui, "Path: ");
    if let Some(ref path) = state.path {
        draw_utils::computed_with_color(ui, path.to_string_lossy(), Color32::WHITE)
    } else {
        draw_utils::computed_with_color(ui, "<Missing path>", Color32::YELLOW)
    };

    ui.end_row();
}
