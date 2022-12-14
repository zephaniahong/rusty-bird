use std::io::{stdout, Stdout};

use crossterm::{
    cursor, execute,
    style::{Color, Print, SetBackgroundColor},
    terminal::{Clear, ClearType},
    Result,
};

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

pub struct FlappyBird {
    image: String,
    position: Position,
}

impl FlappyBird {
    pub fn new(image: String, position: Position) -> Self {
        FlappyBird { image, position }
    }
    pub fn move_bird(&mut self, x: usize, y: usize) {
        self.position = Position::new(x, y);
    }

    pub fn display(&self, mut stdout: &Stdout) -> Result<()> {
        execute!(stdout, Print(&self.image))?;
        Ok(())
    }
}
