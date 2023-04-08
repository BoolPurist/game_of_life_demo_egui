use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use eframe::{
    egui::{self, Button, RichText, Ui, WidgetText},
    epaint::{Color32, Pos2},
};
mod grid;
mod timer;
use grid::{Grid, GridDrawSettings, TextData};
use timer::Timer;

const INITIAL_NAME: &str = "initial.txt";
const ALIVE_CHAR: char = 'x';
const DEAD_CHAR: char = '*';
const MARGIN: f32 = 20.;

fn get_text_input() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(INITIAL_NAME)
}
fn main() {
    let intial_path = get_text_input();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Game of life",
        options,
        Box::new(|_cc| {
            Box::new(GameOfLifeWindow::from_text_file(
                intial_path,
                Duration::from_secs(1),
            ))
        }),
    )
    .unwrap();
}

struct GameOfLifeWindow {
    grid: Grid,
    tick_timer: Timer,
    is_paused: bool,
}

impl GameOfLifeWindow {
    pub fn from_text_file(path: PathBuf, tick: Duration) -> Self {
        let content = std::fs::read_to_string(path).expect("failed to read input text file");
        let text = TextData::new(content, DEAD_CHAR, ALIVE_CHAR);
        let grid = Grid::new(text, GridDrawSettings::default());
        Self {
            grid,
            tick_timer: Timer::new(tick),
            is_paused: false,
        }
    }
}
impl eframe::App for GameOfLifeWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.tick_timer.is_up() {
                self.tick_timer.reset();
                self.grid.tick();
            }

            draw_stats(self, ui);
            draw_buttons(self, ui);

            let y_offset = ui.available_rect_before_wrap().min.y;
            let start = Pos2 {
                x: MARGIN,
                y: y_offset,
            };

            self.grid.draw_at(ui, start);

            ctx.request_repaint();
        });
    }
}

fn draw_buttons(app: &GameOfLifeWindow, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let is_paused = app.is_paused;
        let pause_txt = if is_paused { "Resume" } else { "Pause" };
        _ = ui.button(pause_txt);
        let next_btn = Button::new("Next");
        ui.add_enabled(is_paused, next_btn);
        let reset_btn = Button::new("Reset").fill(Color32::RED);
        ui.add(reset_btn);
    });
    ui.separator();
}

fn draw_stats(app: &GameOfLifeWindow, ui: &mut Ui) {
    egui::Grid::new("Game of life labels")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Passed ticks:");
            computed_value(ui, app.grid.passed_tick().to_string());
            ui.end_row();
            ui.label("Tick rate:");
            computed_value(
                ui,
                format!("{} ms", app.tick_timer.interval_as_ms().to_string()),
            );
            ui.end_row();
        });

    ui.separator();

    fn computed_value(ui: &mut Ui, text: impl Into<WidgetText>) {
        ui.label(text.into().strong());
    }
}
