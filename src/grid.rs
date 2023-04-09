use std::iter::repeat;

use eframe::{
    egui::Ui,
    epaint::{Color32, Pos2, Rect, RectShape, Rounding, Shape, Stroke},
};

mod outer;
mod text_data;
pub use outer::*;
pub use text_data::TextData;
pub mod text_load_error;

const STROKE_WIDTH: f32 = 1.;

#[derive(Clone)]
pub struct Grid {
    all_cells: Vec<LifeCell>,
    drawing: GridDrawSettings,
    height: usize,
    width: usize,
    passed_ticks: usize,
}

impl Grid {
    pub fn new(text: TextData, drawing: GridDrawSettings) -> Self {
        let (height, width) = (text.height(), text.width());
        let mut all_cells = Vec::with_capacity(height * width);

        let initial_cells = all_coords(height, width).map(|(y, x)| text.cell_at_y_x(y, x));
        all_cells.extend(initial_cells);

        Self {
            all_cells,
            height,
            width,
            drawing,
            passed_ticks: 0,
        }
    }

    pub fn draw_at(&self, ui: &mut Ui, start: Pos2) {
        let cell_size = self.drawing.cell_size as f32;
        let (height, width) = (self.height, self.width);
        let mut output = Vec::with_capacity(height * width);

        for (y, x) in all_coords(height, width) {
            let current_cell = *self.all_cells.get(y_x_to_index(width, y, x)).unwrap();
            let y = y as f32;
            let x = x as f32;
            let min_y = (y * cell_size) + start.y;
            let min_x = (x * cell_size) + start.x;
            let max_x = min_x + cell_size;
            let max_y = min_y + cell_size;

            let color = self.get_color_for_cell(current_cell);
            let shape = RectShape {
                rect: Rect {
                    min: Pos2 { x: min_x, y: min_y },
                    max: Pos2 { x: max_x, y: max_y },
                },
                rounding: Rounding::default(),
                fill: color,
                stroke: Stroke {
                    width: STROKE_WIDTH,
                    color: self.drawing.stroke_color,
                },
            };

            output.push(Shape::Rect(shape));
        }

        ui.painter().extend(output);
    }

    pub fn tick(&mut self) {
        let to_apply = self.calcalute_change();

        self.all_cells
            .iter_mut()
            .zip(to_apply.into_iter())
            .for_each(|(old_cell, new_cell)| *old_cell = new_cell);

        self.passed_ticks += 1;
    }

    pub fn passed_tick(&self) -> usize {
        self.passed_ticks
    }

    fn count_alive_cells(&self, y: usize, x: usize) -> usize {
        let (height, width) = (self.height, self.width);
        let left_x = (x + (width - 1)) % width;
        let right_x = (x + 1) % width;
        let top_y = (y + (height - 1)) % height;
        let bottom_y = (y + 1) % height;

        let mut found_alive_adjacant = 0;
        for (y, x) in [
            (top_y, left_x),
            (top_y, x),
            (top_y, right_x),
            (y, left_x),
            (y, right_x),
            (bottom_y, left_x),
            (bottom_y, x),
            (bottom_y, right_x),
        ] {
            let index = y_x_to_index(width, y, x);
            let current_cell = self.all_cells.get(index).unwrap_or_else(|| {
                panic!(
                    "Out of bounds index with x and y: ({},{}), width and height ({}, {}), index {} and length {}",
                    x, y, width, height, index, self.all_cells.len()
                )
            });
            match current_cell {
                LifeCell::Alive => found_alive_adjacant += 1,
                LifeCell::Dead => (),
            };
        }

        found_alive_adjacant
    }

    fn calcalute_change(&self) -> Vec<LifeCell> {
        let mut to_return = Vec::with_capacity(self.all_cells.len());
        for (y, x) in all_coords(self.height, self.width) {
            let cell_count = self.count_alive_cells(y, x);
            let index = y_x_to_index(self.width, y, x);
            let current_cell = self.all_cells.get(index).unwrap();
            let new_cell_val = match (*current_cell, cell_count) {
                (LifeCell::Alive, alive_count) if alive_count < 2 => LifeCell::Dead,
                (LifeCell::Alive, alive_count) if (2..=3).contains(&alive_count) => LifeCell::Alive,
                (LifeCell::Alive, alive_count) if alive_count > 3 => LifeCell::Dead,
                (LifeCell::Dead, 3) => LifeCell::Alive,
                (otherwise, _) => otherwise,
            };

            to_return.push(new_cell_val);
        }
        to_return
    }

    fn get_color_for_cell(&self, cell: LifeCell) -> Color32 {
        match cell {
            LifeCell::Alive => self.drawing.dead_cell_color,
            LifeCell::Dead => self.drawing.alive_cell_color,
        }
    }
}

pub fn y_x_to_index(width: usize, y: usize, x: usize) -> usize {
    (y * width) + x
}

pub fn all_coords(height: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |y| repeat(y).take(width).zip(0..width))
}
