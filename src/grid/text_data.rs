use super::LifeCell;

pub struct TextData {
    dead_char: char,
    alive_char: char,
    width: usize,
    height: usize,
    text_date: Vec<char>,
}

impl TextData {
    pub fn new(text: String, dead_char: char, alive_char: char) -> Self {
        let first_line = text.lines().next().expect("No line supplied").to_owned();
        let width = first_line.len();

        let height = text.lines().count();
        let mut text_date = Vec::with_capacity(height * width);

        for next_line in text.lines() {
            validate_row(next_line, width, dead_char, alive_char);
            let next_line: Vec<char> = next_line.chars().collect();
            text_date.extend_from_slice(&next_line);
        }

        return Self {
            dead_char,
            alive_char,
            width,
            height,
            text_date,
        };

        fn validate_row(row: &str, width: usize, dead_char: char, alive_char: char) {
            if row.len() != width {
                panic!("Invalid text unequal width {} and row {}", width, row.len());
            } else if !row.chars().all(|sym| sym == dead_char || sym == alive_char) {
                panic!(
                    "Invalid text data with not all rows having only chars for dead and live cell"
                )
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
