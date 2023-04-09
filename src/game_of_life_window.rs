use crate::open_view::OpenView;
use crate::CurrentView;
use eframe::egui;
pub struct GameOfLifeWindow {
    view: CurrentView,
}

impl Default for GameOfLifeWindow {
    fn default() -> Self {
        Self {
            view: CurrentView::Open(OpenView::default()),
        }
    }
}

impl eframe::App for GameOfLifeWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.view {
            CurrentView::Game(game_view) => {
                game_view.tick_if_up();
                if let Some(open_view) = game_view.draw(ui) {
                    self.view = CurrentView::Open(open_view);
                }
                ctx.request_repaint();
            }
            CurrentView::Open(open_view) => {
                if let Some(new_view) = open_view.draw(ctx, ui) {
                    self.view = new_view;
                }
            }
        });
    }
}
