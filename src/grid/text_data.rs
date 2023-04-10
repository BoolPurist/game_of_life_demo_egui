use std::{io::ErrorKind, path::Path};

use crate::grid::text_load_error::TextLoadError;

use super::{dead_alive_char_cells::DeadAliveCharCell, LifeCell};

pub struct TextData {
    dead_char: char,
    alive_char: char,
    width: usize,
    height: usize,
    text_date: Vec<char>,
}

impl TextData {
    pub fn new(path: &Path, dead_alive: DeadAliveCharCell) -> Result<Self, TextLoadError> {
        let text = match std::fs::read_to_string(path) {
            Err(error) if error.kind() == ErrorKind::NotFound => {
                Err(TextLoadError::NoFileFound(path.to_owned()))
            }
            Err(error) => Err(TextLoadError::IoError(error)),
            Ok(content) => Ok(content),
        }?;
        let first_line = text.lines().next().expect("No line supplied").to_owned();
        let width = first_line.len();

        let height = text.lines().count();
        let mut text_date = Vec::with_capacity(height * width);

        let (dead_char, alive_char) = (dead_alive.dead(), dead_alive.alive());
        for next_line in text.lines() {
            validate_row(next_line, width, dead_char, alive_char)?;
            let next_line: Vec<char> = next_line.chars().collect();
            text_date.extend_from_slice(&next_line);
        }

        return Ok(Self {
            dead_char,
            alive_char,
            width,
            height,
            text_date,
        });

        fn validate_row(
            row: &str,
            width: usize,
            dead_char: char,
            alive_char: char,
        ) -> Result<(), TextLoadError> {
            if row.len() != width {
                Err(TextLoadError::UnEqualWidth(width))
            } else if let Some(invalid_char) = row
                .chars()
                .find(|&sym| sym != dead_char && sym != alive_char)
            {
                Err(TextLoadError::NotValidCellChar(invalid_char))
            } else {
                Ok(())
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cell_at_y_x(&self, y: usize, x: usize) -> LifeCell {
        let index = super::y_x_to_index(self.width, y, x);
        let char_cell = self.text_date.get(index).unwrap();
        match *char_cell {
            cell if self.dead_char == cell => LifeCell::Dead,
            cell if self.alive_char == cell => LifeCell::Alive,
            cell => unreachable!("char: {}", cell),
        }
    }
}
