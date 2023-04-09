use super::{DataFileState, GatheredOpenViewData, OpenView};
use crate::grid::{Grid, GridDrawSettings, TextData};
use crate::{constans::*, draw_utils};
use crate::{game_view::GameView, CurrentView};
use eframe::egui::{self, Button, Ui};
use eframe::epaint::Color32;
use egui_file::FileDialog;

const BTN_TEXT_LOAD: &str = "Load";
const BTN_TEXT_PLAY: &str = "Play";
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
    let mut clicked_play = false;

    ui.horizontal(|ui| {
        if ui.button("Choose").clicked() {
            let mut dialog = FileDialog::open_file(None);
            dialog.open();
            state.open_file_dialog = Some(dialog);
        }

        if let Some(dialog) = &mut state.open_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(path) = dialog.path() {
                    state.game_file_state = DataFileState::Choosen { path }
                }
            }
        }

        let (enable_load_button, enable_play_button) = match &state.game_file_state {
            DataFileState::NotChoosen => (false, false),
            DataFileState::Choosen { .. } => (true, false),
            DataFileState::Invalid { .. } => (false, false),
            DataFileState::Loaded { .. } => (false, true),
        };
        if enable_load_button {
            clicked_load = ui
                .add_enabled(enable_load_button, Button::new(BTN_TEXT_LOAD))
                .clicked();
        }
        if enable_play_button {
            clicked_play = ui
                .add_enabled(enable_play_button, Button::new(BTN_TEXT_PLAY))
                .clicked();
        }
    });

    if clicked_play {
        if let DataFileState::Loaded { .. } = &state.game_file_state {
            if let DataFileState::Loaded { game, path } = std::mem::take(&mut state.game_file_state)
            {
                let gathered = GatheredOpenViewData {
                    alive_char_code: state.alive_char_code,
                    dead_char_code: state.dead_char_code,
                    game,
                    path,
                };
                return Some(CurrentView::Game(GameView::new(gathered, TICK_DURATION)));
            }
        }
    } else if clicked_load {
        if let DataFileState::Choosen { .. } = &mut state.game_file_state {
            if let DataFileState::Choosen { path } = std::mem::take(&mut state.game_file_state) {
                match TextData::new(&path, state.dead_char_code, state.alive_char_code) {
                    Err(error) => state.game_file_state = DataFileState::Invalid { path, error },
                    Ok(data) => {
                        let game = Grid::new(data, GridDrawSettings::default());
                        state.game_file_state = DataFileState::Loaded { path, game };
                    }
                }
            }
        }
    }

    None
}

fn draw_path_and_chars_for_text(state: &mut OpenView, ui: &mut Ui) {
    draw_utils::draw_grid(ui, "Input grid", |ui| match &state.game_file_state {
        DataFileState::NotChoosen => draw_path_line(ui, "<Missing Path>", Color32::YELLOW),
        DataFileState::Choosen { path, .. } => {
            draw_path_line(ui, &path.to_string_lossy(), Color32::WHITE);
            draw_cell_fields(state, ui);
        }
        DataFileState::Invalid { error, path } => {
            draw_path_line(ui, &path.to_string_lossy(), Color32::RED);
            draw_cell_fields(state, ui);
            draw_utils::computed_with_color(ui, error.to_string(), Color32::RED);
        }
        DataFileState::Loaded { path, .. } => {
            draw_path_line(ui, &path.to_string_lossy(), Color32::GREEN);
            draw_cell_fields(state, ui);
        }
    });

    fn draw_cell_fields(state: &OpenView, ui: &mut Ui) {
        ui.label("Char alive cell:");
        draw_utils::computed_value(ui, state.alive_char_code.to_string());
        ui.end_row();

        ui.label("Char dead cell:");
        draw_utils::computed_value(ui, state.dead_char_code.to_string());
        ui.end_row();
    }
}

fn draw_path_line(ui: &mut Ui, message: &str, color: Color32) {
    draw_utils::computed_value(ui, "Path: ");
    draw_utils::computed_with_color(ui, message, color);

    ui.end_row();
}
