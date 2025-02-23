use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;
use super::{Color, Move, Piece};

#[derive(Clone, Copy, Debug)]
pub struct King {
    color: Color,
    position: Position,
}

impl Piece for King {
    fn new(position: Position, color: Color) -> Self {
        Self {
            color,
            position,
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn possible_moves(&self, board: &Board, position: &Position) -> HashSet<Move> {
        todo!();
        // index = row * ROWS + column
        // let row: usize = index / ROWS;
        // let column: usize = index % COLUMNS;
    }
}
