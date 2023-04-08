mod game_of_life_window;
mod game_view;
mod grid;
mod timer;

use game_of_life_window::GameOfLifeWindow;
use grid::{GridDrawSettings, TextData};

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use eframe::egui;

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
        initial_window_size: Some(egui::vec2(640.0, 480.)),
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
