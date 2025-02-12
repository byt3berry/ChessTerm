use std::fmt::Display;

use square::Square;
use crate::{pieces::Color, player::Player};

pub mod square;

pub const ROWS: usize = 8;
pub const COLUMNS: usize = 8;

#[derive(Debug)]
pub struct Board {
    players: [Player; 2],
    board: [Square; ROWS*COLUMNS],
}

impl Board {
    pub fn new() -> Self {
        Self {
            players: [Player::new(Color::WHITE), Player::new(Color::BLACK)],
            board: [Square::default(); ROWS*COLUMNS],
        }
    }

    pub fn get(&self, x: usize, y:usize) -> Option<&Square> {
        self.board.get(x * ROWS + y)
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
