use crate::game_view::GameView;
use eframe::egui;
use std::{path::PathBuf, time::Duration};

pub struct GameOfLifeWindow {
    view: GameView,
}

impl GameOfLifeWindow {
    pub fn from_text_file(path: PathBuf, tick: Duration) -> Self {
        let mut view = GameView::new(path, tick);
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
