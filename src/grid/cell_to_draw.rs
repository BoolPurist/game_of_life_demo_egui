use eframe::epaint::{Color32, RectShape};

use super::LifeCell;

#[derive(Clone)]
pub struct CellToDraw {
    cell: LifeCell,
    rect: RectShape,
}

impl CellToDraw {
    pub fn new(cell: LifeCell, rect: RectShape) -> Self {
        Self { cell, rect }
    }
    pub fn apply_cell_color(&mut self, new_cell: LifeCell, new_color: Color32) {
        self.cell = new_cell;
        self.rect.fill = new_color;
    }

    pub fn copy_rect(&self) -> RectShape {
        self.rect
    }

    pub fn cell(&self) -> LifeCell {
        self.cell
    }
}
