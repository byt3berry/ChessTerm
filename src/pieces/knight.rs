use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;
use super::{Color, Move, Piece};

#[derive(Clone, Copy, Debug)]
pub struct Knight {
    color: Color,
    position: Position,
}

impl Piece for Knight {
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
        // let mut output: Vec<&Square> = Vec::new();

        // if let Some(square) = board.get(row + 1, column + 2) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //     } else {
        //         output.push(square);
        //     }
        // }

        // if let Some(square) = board.get(row - 1, column + 2) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //     } else {
        //         output.push(square);
        //     }
        // }

        // if let Some(square) = board.get(row + 1, column - 2) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //     } else {
        //         output.push(square);
        //     }
        // }

        // if let Some(square) = board.get(row - 1, column - 2) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //     } else {
        //         output.push(square);
        //     }
        // }

        // if let Some(square) = board.get(row + 2, column + 1) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //     } else {
        //         output.push(square);
        //     }
        // }

        // if let Some(square) = board.get(row - 2, column + 1) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //     } else {
        //         output.push(square);
        //     }
        // }

        // if let Some(square) = board.get(row + 2, column - 1) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //     } else {
        //         output.push(square);
        //     }
        // }

        // if let Some(square) = board.get(row - 2, column - 1) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //     } else {
        //         output.push(square);
        //     }
        // }

        // return output;
    }
}
