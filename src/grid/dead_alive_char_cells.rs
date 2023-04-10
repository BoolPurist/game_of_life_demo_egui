use getset::CopyGetters;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum InvalidCharCell {
    #[error("Text for dead cell should only contain one char")]
    TooLongForDead,
    #[error("Text for alive cell should only contain one char")]
    TooLongForAlive,
}
#[derive(CopyGetters)]
#[getset(get_copy = "pub")]
pub struct DeadAliveCharCell {
    alive: char,
    dead: char,
}

impl DeadAliveCharCell {
    pub fn new(dead: &str, alive: &str) -> Result<Self, InvalidCharCell> {
        let dead = get_only_first_char(dead).ok_or(InvalidCharCell::TooLongForDead)?;
        let alive = get_only_first_char(alive).ok_or(InvalidCharCell::TooLongForAlive)?;
        return Ok(Self { dead, alive });

        fn get_only_first_char(input: &str) -> Option<char> {
            let input = input.trim();
            if input.len() == 1 {
                Some(input.chars().next().unwrap())
            } else {
                None
            }
        }
    }
}
