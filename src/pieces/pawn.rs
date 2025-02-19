use crate::board::{Board, COLUMNS, ROWS};
use crate::board::square::Square;

use super::{Color, Piece};

#[derive(Clone, Copy, Debug)]
pub struct Pawn {
    color: Color,
    en_passant: bool,
    has_moved: bool,
}

impl Piece for Pawn {
    fn new(index: usize) -> Self {
        let color: Color = if index / ROWS <= 1 {
            Color::BLACK
        } else if index / ROWS >= 6 {
            Color::WHITE
        } else {
            panic!("Invalid index: {index}")
        };

        Self {
            color,
            en_passant: false,
            has_moved: false,
        }
    }

    fn move_to(&self, square: Square) {
        todo!()
    }

    fn possible_moves<'a>(&'a self, board: &'a Board, index: usize) -> Vec<&'a Square> {
        // index = row * ROWS + column
        let row: usize = index / ROWS;
        let column: usize = index % COLUMNS;
        todo!()
    }

    fn color(&self) -> Color {
        self.color
    }
}
