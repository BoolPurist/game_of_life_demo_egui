use std::error::Error;
use std::path::Path;

use super::{DataFileState, GatheredOpenViewData, OpenView};
use crate::grid::{DeadAliveCharCell, Grid, GridDrawSettings, TextData};
use crate::open_view::SelectedTime;
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
            DataFileState::Invalid { .. } => (true, false),
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
                match state.time_interval.parse() {
                    Ok(valid_number) => {
                        let time_interval =
                            super::time_unit_from_selection(state.selected_time, valid_number);
                        let gathered = GatheredOpenViewData {
                            alive_char_code: state.alive_char_code,
                            dead_char_code: state.dead_char_code,
                            game,
                            path,
                            selected_time: state.selected_time,
                            time_interval,
                        };
                        return Some(CurrentView::Game(GameView::new(gathered)));
                    }
                    Err(error) => {
                        state.game_file_state = DataFileState::Invalid {
                            path,
                            error: error.into(),
                        };
                    }
                }
            }
        }
    } else if clicked_load {
        if let DataFileState::Choosen { .. } | DataFileState::Invalid { .. } =
            &mut state.game_file_state
        {
            if let DataFileState::Choosen { path } | DataFileState::Invalid { path, .. } =
                std::mem::take(&mut state.game_file_state)
            {
                match DeadAliveCharCell::new(&state.dead_char_input, &state.alive_char_input) {
                    Err(error) => {
                        state.game_file_state = DataFileState::Invalid {
                            path,
                            error: error.into(),
                        };
                    }
                    Ok(valid_dead_alive) => match TextData::new(&path, valid_dead_alive) {
                        Err(error) => {
                            state.game_file_state = DataFileState::Invalid {
                                path,
                                error: error.into(),
                            }
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
    let mut dead_alive_input = (
        String::default(),
        String::default(),
        String::default(),
        SelectedTime::Seconds,
        None,
    );
    draw_utils::draw_grid(ui, "Input grid", |ui| {
        dead_alive_input = match &state.game_file_state {
            DataFileState::NotChoosen => {
                draw_path_line(ui, MISSING_PATH_TXT, WARN_COLOR);
                (
                    state.dead_char_input.clone(),
                    state.alive_char_input.clone(),
                    state.time_interval.clone(),
                    state.selected_time,
                    None,
                )
            }
            DataFileState::Choosen { path, .. } => {
                draw_path_line(ui, &path.to_string_lossy(), NORMAL_COLOR);
                draw_cell_fields(state, ui)
            }
            DataFileState::Invalid { error, path } => draw_error_case(ui, state, error, path),
            DataFileState::Loaded { path, .. } => {
                draw_path_line(ui, &path.to_string_lossy(), SUCCESS_COLOR);
                draw_cell_fields(state, ui)
            }
        };
    });

    let (dead, alive, time, selected_time, error) = dead_alive_input;

    state.dead_char_input = dead;
    state.alive_char_input = alive;
    state.selected_time = selected_time;
    state.time_interval = time;

    if let Some(error_message) = error {
        draw_utils::computed_with_color(ui, error_message, ERR_COLOR);
    }

    fn draw_cell_fields(
        state: &OpenView,
        ui: &mut Ui,
    ) -> (String, String, String, SelectedTime, Option<String>) {
        let (dead, alive) = (
            draw_input_single_line(ui, "Char dead cell:", state.dead_char_input.clone()),
            draw_input_single_line(ui, "Char alive cell:", state.alive_char_input.clone()),
        );

        let mut selected_time = state.selected_time;
        egui::ComboBox::from_label("Determine the used time unit")
            .selected_text(format!("{:?}", selected_time))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected_time, SelectedTime::Seconds, "Seconds");
                ui.selectable_value(&mut selected_time, SelectedTime::MsSeconds, "Miliseconds");
            });
        ui.end_row();

        return (
            dead,
            alive,
            draw_input_single_line(ui, "Time:", state.time_interval.clone()),
            selected_time,
            None,
        );

        fn draw_input_single_line(ui: &mut Ui, label: &str, mut base: String) -> String {
            ui.label(label);
            ui.text_edit_singleline(&mut base);
            ui.end_row();

            base
        }
    }

    fn draw_error_case(
        ui: &mut Ui,
        state: &OpenView,
        error: &impl Error,
        path: &Path,
    ) -> (String, String, String, SelectedTime, Option<String>) {
        draw_path_line(ui, &path.to_string_lossy(), ERR_COLOR);
        let to_return = draw_cell_fields(state, ui);

        (
            to_return.0,
            to_return.1,
            to_return.2,
            to_return.3,
            Some(error.to_string()),
        )
    }
}

fn draw_path_line(ui: &mut Ui, message: &str, color: Color32) {
    draw_utils::computed_value(ui, "Path: ");
    draw_utils::computed_with_color(ui, message, color);

    ui.end_row();
}
