use std::error::Error;
use std::path::Path;

use super::{DataFileState, GatheredOpenViewData, OpenView};
use crate::grid::{DeadAliveCharCell, Grid, GridDrawSettings, TextData};
use crate::{constans::*, draw_utils};
use crate::{game_view::GameView, CurrentView};
use eframe::egui::{self, Button, Ui};
use eframe::epaint::Color32;
use egui_file::FileDialog;

pub fn draw_input_mask(
    state: &mut OpenView,
    ui: &mut Ui,
    ctx: &egui::Context,
) -> Option<CurrentView> {
    draw_path_and_chars_for_text(state, ui);

    ui.separator();
    draw_buttons(state, ui, ctx)
}

fn draw_buttons(state: &mut OpenView, ui: &mut Ui, ctx: &egui::Context) -> Option<CurrentView> {
    let mut clicked_load = false;
    let mut clicked_play = false;

    ui.horizontal(|ui| {
        if ui
            .button(draw_utils::create_rich_text(BTN_CHOOSE_TXT))
            .clicked()
        {
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
            DataFileState::Invalid { .. } | DataFileState::InvalidChars { .. } => (true, false),
            DataFileState::Loaded { .. } => (false, true),
        };

        clicked_load = ui
            .add_enabled(
                enable_load_button,
                Button::new(draw_utils::create_rich_text(BTN_TEXT_LOAD)),
            )
            .clicked();
        clicked_play = ui
            .add_enabled(
                enable_play_button,
                Button::new(draw_utils::create_rich_text(BTN_TEXT_PLAY)),
            )
            .clicked();
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
        if let DataFileState::Choosen { .. }
        | DataFileState::InvalidChars { .. }
        | DataFileState::Invalid { .. } = &mut state.game_file_state
        {
            if let DataFileState::Choosen { path }
            | DataFileState::InvalidChars { path, .. }
            | DataFileState::Invalid { path, .. } = std::mem::take(&mut state.game_file_state)
            {
                match DeadAliveCharCell::new(&state.dead_char_input, &state.alive_char_input) {
                    Err(error) => {
                        state.game_file_state = DataFileState::InvalidChars { path, error };
                    }
                    Ok(valid_dead_alive) => match TextData::new(&path, valid_dead_alive) {
                        Err(error) => {
                            state.game_file_state = DataFileState::Invalid { path, error }
                        }
                        Ok(data) => {
                            let game = Grid::new(data, GridDrawSettings::default());
                            state.game_file_state = DataFileState::Loaded { path, game };
                        }
                    },
                }
            }
        }
    }

    None
}

fn draw_path_and_chars_for_text(state: &mut OpenView, ui: &mut Ui) {
    let mut dead_alive_input = (String::default(), String::default(), None);
    draw_utils::draw_grid(ui, "Input grid", |ui| {
        dead_alive_input = match &state.game_file_state {
            DataFileState::NotChoosen => {
                draw_path_line(ui, MISSING_PATH_TXT, WARN_COLOR);
                (Default::default(), Default::default(), None)
            }
            DataFileState::Choosen { path, .. } => {
                draw_path_line(ui, &path.to_string_lossy(), NORMAL_COLOR);
                draw_cell_fields(state, ui)
            }
            DataFileState::Invalid { error, path } => draw_error_case(ui, state, error, path),
            DataFileState::InvalidChars { error, path } => draw_error_case(ui, state, error, path),
            DataFileState::Loaded { path, .. } => {
                draw_path_line(ui, &path.to_string_lossy(), SUCCESS_COLOR);
                draw_cell_fields(state, ui)
            }
        };
    });

    let (dead, alive, error) = dead_alive_input;

    state.dead_char_input = dead;
    state.alive_char_input = alive;

    if let Some(error_message) = error {
        draw_utils::computed_with_color(ui, error_message, ERR_COLOR);
    }

    fn draw_cell_fields(state: &OpenView, ui: &mut Ui) -> (String, String, Option<String>) {
        ui.label("Char dead cell:");

        let mut input_dead_cell = state.dead_char_input.clone();
        ui.text_edit_singleline(&mut input_dead_cell);
        ui.end_row();

        ui.label("Char alive cell:");
        let mut input_alive_cell = state.alive_char_input.clone();
        ui.text_edit_singleline(&mut input_alive_cell);
        ui.end_row();

        (input_dead_cell, input_alive_cell, None)
    }

    fn draw_error_case(
        ui: &mut Ui,
        state: &OpenView,
        error: &impl Error,
        path: &Path,
    ) -> (String, String, Option<String>) {
        draw_path_line(ui, &path.to_string_lossy(), ERR_COLOR);
        let to_return = draw_cell_fields(state, ui);
        (to_return.0, to_return.1, Some(error.to_string()))
    }
}

fn draw_path_line(ui: &mut Ui, message: &str, color: Color32) {
    draw_utils::computed_value(ui, "Path: ");
    draw_utils::computed_with_color(ui, message, color);

    ui.end_row();
}
