use eframe::egui::Ui;
use eframe::epaint::Pos2;

use crate::constans::*;
use crate::grid::Grid;
use crate::timer::Timer;
use crate::GridDrawSettings;
use crate::TextData;
use std::path::PathBuf;
use std::time::Duration;
mod drawing;

pub struct GameView {
    initial_grid: Grid,
    grid: Grid,
    tick_timer: Timer,
    is_paused: bool,
}

impl GameView {
    pub fn new(path: PathBuf, tick: Duration) -> Self {
        let content = std::fs::read_to_string(path).expect("failed to read input text file");
        let text = TextData::new(content, DEAD_CHAR, ALIVE_CHAR);
        let grid = Grid::new(text, GridDrawSettings::default());
        Self {
            initial_grid: grid.clone(),
            grid,
            tick_timer: Timer::new(tick),
            is_paused: false,
        }
    }
    pub fn reset(&mut self) {
        self.pause();
        self.tick_timer.reset();
        self.grid = self.initial_grid.clone();
    }

    pub fn pause(&mut self) {
        self.tick_timer.pause();
        self.is_paused = true;
    }
    pub fn resume(&mut self) {
        self.tick_timer.resume();
        self.is_paused = false;
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn toogle_pause_resume(&mut self) {
        if self.is_paused {
            self.resume();
        } else {
            self.pause();
        }
    }

    pub fn tick_if_up(&mut self) {
        if self.tick_timer.is_up() {
            self.tick_timer.reset_time();
            self.grid.tick();
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        drawing::draw_stats(self, ui);
        drawing::draw_buttons(self, ui);

        let y_offset = ui.available_rect_before_wrap().min.y;
        let start = Pos2 {
            x: MARGIN,
            y: y_offset,
        };

        self.grid.draw_at(ui, start);
    }
}
