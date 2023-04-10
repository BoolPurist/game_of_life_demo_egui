use super::GameView;
use crate::constans::*;
use crate::draw_utils;
use crate::open_view::OpenView;
use eframe::{egui::Ui, epaint::Color32};

pub fn draw_buttons(app: &mut GameView, ui: &mut Ui) -> Option<OpenView> {
    let mut to_return = None;

    ui.horizontal(|ui| {
        let pause_txt = if app.is_paused {
            BTN_RESUME_TXT
        } else {
            BTN_PAUSE_TXT
        };
        if ui.button(draw_utils::create_rich_text(pause_txt)).clicked() {
            app.toogle_pause_resume();
        }

        {
            let next_btn = draw_utils::button(BTN_NEXT_TXT);
            let next_btn = ui.add_enabled(app.is_paused, next_btn);
            if next_btn.clicked() {
                app.grid.tick();
            }
        }

        let reset_btn = draw_utils::button(BTN_RESET_TXT).fill(ERR_COLOR);
        if ui.add(reset_btn).clicked() {
            app.reset();
        };
        let back_btn = draw_utils::button_with_color(BTN_BACK_TXT, Color32::BLACK).fill(WARN_COLOR);
        if ui.add(back_btn).clicked() {
            app.pause();
            to_return = Some(OpenView::new(app.previous_view.clone()));
        };
    });

    ui.separator();

    to_return
}

pub fn draw_stats(app: &GameView, ui: &mut Ui) {
    draw_utils::draw_grid(ui, "Game of life labels", |ui| {
        ui.label(draw_utils::create_rich_text("Passed ticks:"));
        draw_utils::computed_value(ui, &app.grid.passed_tick().to_string());
        ui.end_row();

        ui.label(draw_utils::create_rich_text("Tick rate:"));
        draw_utils::computed_value(ui, &format!("{} ms", app.tick_timer.interval_as_ms()));
        ui.end_row();

        ui.label(draw_utils::create_rich_text("State:"));
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
