use eframe::egui::Ui;
use eframe::epaint::Pos2;

use crate::constans::*;
use crate::grid::Grid;
use crate::open_view::{GatheredOpenViewData, OpenView};
use crate::timer::Timer;
mod drawing;

pub struct GameView {
    grid: Grid,
    tick_timer: Timer,
    is_paused: bool,
    previous_view: GatheredOpenViewData,
}

impl GameView {
    pub fn new(previous_view: GatheredOpenViewData) -> Self {
        let mut slf = Self {
            grid: previous_view.clone_game(),
            tick_timer: Timer::new(previous_view.time_interval.into()),
            previous_view,
            is_paused: false,
        };

        slf.reset();

        slf
    }
    pub fn reset(&mut self) {
        self.pause();
        self.tick_timer.reset();
        self.grid = self.previous_view.clone_game();
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

    pub fn draw(&mut self, ui: &mut Ui) -> Option<OpenView> {
        drawing::draw_stats(self, ui);
        let to_return = drawing::draw_buttons(self, ui);

        let y_offset = ui.available_rect_before_wrap().min.y;
        let start = Pos2 {
            x: MARGIN,
            y: y_offset,
        };

        self.grid.draw_at(ui, start);

        to_return
    }
}
