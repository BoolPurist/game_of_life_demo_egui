use super::GameView;
use crate::constans::*;
use crate::draw_utils;
use eframe::{
    egui::{Button, Ui},
    epaint::Color32,
};

pub fn draw_buttons(app: &mut GameView, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let pause_txt = if app.is_paused {
            BTN_RESUME_TXT
        } else {
            BTN_PAUSE_TXT
        };
        if ui.button(pause_txt).clicked() {
            app.toogle_pause_resume();
        }

        {
            let next_btn = Button::new(BTN_NEXT_TXT);
            let next_btn = ui.add_enabled(app.is_paused, next_btn);
            if next_btn.clicked() {
                app.grid.tick();
            }
        }

        let reset_btn = Button::new(BTN_RESET_TXT).fill(Color32::RED);
        if ui.add(reset_btn).clicked() {
            app.reset();
        };
    });
    ui.separator();
}

pub fn draw_stats(app: &GameView, ui: &mut Ui) {
    draw_utils::draw_grid(ui, "Game of life labels", |ui| {
        ui.label("Passed ticks:");
        draw_utils::computed_value(ui, app.grid.passed_tick().to_string());
        ui.end_row();

        ui.label("Tick rate:");
        draw_utils::computed_value(ui, format!("{} ms", app.tick_timer.interval_as_ms()));
        ui.end_row();

        ui.label("State:");
        let (state_txt, state_color) = if app.is_paused() {
            ("Paused", Color32::YELLOW)
        } else {
            ("Running", Color32::GREEN)
        };
        draw_utils::computed_with_color(ui, state_txt, state_color);
        ui.end_row();
    });

    ui.separator();
}
