use std::error::Error;
use std::path::{Path, PathBuf};

use super::{DataFileState, GatheredOpenViewData, OpenView, TimeUnit};
use crate::grid::{DeadAliveCharCell, Grid, GridDrawSettings, TextData};
use crate::open_view::{SelectedTime, ValidationError};
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
    let mut clicked_play = false;

    ui.horizontal(|ui| {
        file_dialog_for_game_file(ui, ctx, state);
        clicked_play = draw_play_btns(ui, state);
    });

    if clicked_play {
        match try_to_play_game(state) {
            Err(error) => {
                state.game_file_state = error;
            }
            Ok(to_maybe_play) => return to_maybe_play,
        }
    }

    return None;

    fn file_dialog_for_game_file(ui: &mut Ui, ctx: &egui::Context, state: &mut OpenView) {
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
    }

    fn draw_play_btns(ui: &mut Ui, state: &OpenView) -> bool {
        let enable_play_button = match &state.game_file_state {
            DataFileState::NotChoosen => false,
            DataFileState::Choosen { .. } => true,
            DataFileState::Invalid { .. } => true,
        };

        ui.add_enabled(
            enable_play_button,
            Button::new(draw_utils::create_rich_text(BTN_TEXT_PLAY)),
        )
        .clicked()
    }

    fn try_to_play_game(state: &mut OpenView) -> Result<Option<CurrentView>, DataFileState> {
        let Some(path) = try_query_loaded_file(state) else {
            return Ok(None);
        };

        let valid_number = try_parse_interval_time(state, &path)?;
        let dead_alive_chars = validate_given_chars_for_game(state, &path)?;
        let text_data = validate_file_content(dead_alive_chars, &path)?;
        let time_interval = time_unit_from_selection(state.selected_time, valid_number);
        let game = Grid::new(text_data, GridDrawSettings::default());

        let gathered = GatheredOpenViewData {
            alive_char_code: state.alive_char_code,
            dead_char_code: state.dead_char_code,
            game,
            path,
            selected_time: state.selected_time,
            time_interval,
        };
        return Ok(Some(CurrentView::Game(GameView::new(gathered))));

        fn try_query_loaded_file(state: &mut OpenView) -> Option<PathBuf> {
            if let DataFileState::Choosen { .. } | DataFileState::Invalid { .. } =
                &state.game_file_state
            {
                if let DataFileState::Choosen { path } | DataFileState::Invalid { path, .. } =
                    std::mem::take(&mut state.game_file_state)
                {
                    return Some(path);
                }
            }

            None
        }

        fn try_parse_interval_time(
            state: &mut OpenView,
            path: &Path,
        ) -> Result<u32, DataFileState> {
            match state.time_interval.parse() {
                Ok(valid_number) => Ok(valid_number),
                Err(error) => Err(DataFileState::Invalid {
                    path: path.to_owned(),
                    error: error.into(),
                }),
            }
        }
        fn validate_file_content(
            dead_alive_cell_chars: DeadAliveCharCell,
            path: &Path,
        ) -> Result<TextData, (PathBuf, ValidationError)> {
            match TextData::new(path, dead_alive_cell_chars) {
                Err(error) => Err((path.to_owned(), error.into())),
                Ok(data) => Ok(data),
            }
        }
    }

    fn validate_given_chars_for_game(
        state: &mut OpenView,
        path: &Path,
    ) -> Result<DeadAliveCharCell, (PathBuf, ValidationError)> {
        match DeadAliveCharCell::new(&state.dead_char_input, &state.alive_char_input) {
            Err(error) => Err((path.to_owned(), error.into())),
            Ok(valid_dead_alive) => Ok(valid_dead_alive),
        }
    }
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
            .selected_text(format!("{}", selected_time))
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

fn time_unit_from_selection(selected_kind: SelectedTime, amount: u32) -> TimeUnit {
    match selected_kind {
        SelectedTime::Seconds => TimeUnit::Seconds(amount),
        SelectedTime::MsSeconds => TimeUnit::MsSeconds(amount),
    }
}
