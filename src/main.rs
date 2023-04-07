use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use eframe::{egui, epaint::Pos2};
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
}

impl GameOfLifeWindow {
    pub fn from_text_file(path: PathBuf, tick: Duration) -> Self {
        let content = std::fs::read_to_string(path).expect("failed to read input text file");
        let text = TextData::new(content, DEAD_CHAR, ALIVE_CHAR);
        let grid = Grid::new(text, GridDrawSettings::default());
        Self {
            grid,
            tick_timer: Timer::new(tick),
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

            ui.label(format!("Passed ticks: {}", self.grid.passed_tick()));
            ui.label(format!(
                "Tick rate: {} mili seconds",
                self.tick_timer.interval_as_ms()
            ));
            ui.separator();

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
