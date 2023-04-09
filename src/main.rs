mod draw_utils;
mod game_of_life_window;
mod game_view;
mod grid;
mod open_view;
mod timer;

use crate::open_view::OpenView;
use game_of_life_window::GameOfLifeWindow;
use game_view::GameView;

use std::path::{Path, PathBuf};

use eframe::egui;

pub enum CurrentView {
    Game(GameView),
    Open(OpenView),
}

mod constans {
    use std::time::Duration;

    pub const _INITIAL_NAME: &str = "initial.txt";
    pub const ALIVE_CHAR: char = 'x';
    pub const DEAD_CHAR: char = '*';
    pub const MARGIN: f32 = 20.;
    pub const TICK_DURATION: Duration = Duration::from_secs(1);
    pub const BTN_RESUME_TXT: &str = "Resume";
    pub const BTN_PAUSE_TXT: &str = "Pause";
    pub const BTN_NEXT_TXT: &str = "Next";
    pub const BTN_RESET_TXT: &str = "Reset";
    pub const GRID_SPACEING: &[f32; 2] = &[40.0, 4.0];
}

fn _get_text_input() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(constans::_INITIAL_NAME)
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.)),
        ..Default::default()
    };
    eframe::run_native(
        "Game of life",
        options,
        Box::new(|_cc| Box::<GameOfLifeWindow>::default()),
    )
    .unwrap();
}
