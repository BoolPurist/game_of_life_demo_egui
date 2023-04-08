use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use eframe::egui;
mod game_view_state;
mod grid;
mod timer;

use game_view_state::GameViewState;
use grid::{GridDrawSettings, TextData};

mod constans {

    pub const INITIAL_NAME: &str = "initial.txt";

    pub const ALIVE_CHAR: char = 'x';
    pub const DEAD_CHAR: char = '*';
    pub const MARGIN: f32 = 20.;
    pub const BTN_RESUME_TXT: &str = "Resume";
    pub const BTN_PAUSE_TXT: &str = "Pause";
    pub const BTN_NEXT_TXT: &str = "Next";
    pub const BTN_RESET_TXT: &str = "Reset";
}

fn get_text_input() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(constans::INITIAL_NAME)
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
    view: GameViewState,
}
impl GameOfLifeWindow {
    pub fn from_text_file(path: PathBuf, tick: Duration) -> Self {
        let mut view = GameViewState::new(path, tick);
        view.reset();
        Self { view }
    }
}

impl eframe::App for GameOfLifeWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.view.tick_if_up();
            self.view.draw(ui);

            ctx.request_repaint();
        });
    }
}
