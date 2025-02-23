use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;
use super::{Color, Move, Piece};

#[derive(Clone, Copy, Debug)]
pub struct Queen {
    color: Color,
    position: Position,
}

impl Piece for Queen {
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
        // let mut offset;

        // offset = 1;
        // while let Some(square) = board.get(row + offset, column + offset) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //         break;
        //     } else {
        //         output.push(square);
        //     }
        // }

        // offset = 1;
        // while let Some(square) = board.get(row - offset, column + offset) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //         break;
        //     } else {
        //         output.push(square);
        //     }
        // }

        // offset = 1;
        // while let Some(square) = board.get(row + offset, column - offset) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //         break;
        //     } else {
        //         output.push(square);
        //     }
        // }

        // offset = 1;
        // while let Some(square) = board.get(row - offset, column - offset) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //         break;
        //     } else {
        //         output.push(square);
        //     }
        // }

        // offset = 1;
        // while let Some(square) = board.get(row + offset, column) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //         break;
        //     } else {
        //         output.push(square);
        //     }
        // }

        // offset = 1;
        // while let Some(square) = board.get(row - offset, column) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //         break;
        //     } else {
        //         output.push(square);
        //     }
        // }

        // offset = 1;
        // while let Some(square) = board.get(row, column + offset) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //         break;
        //     } else {
        //         output.push(square);
        //     }
        // }

        // offset = 1;
        // while let Some(square) = board.get(row, column - offset) {
        //     if let Some(piece) = square.piece() {
        //         if piece.color() != self.color() {
        //             output.push(square);
        //         }
        //         break;
        //     } else {
        //         output.push(square);
        //     }
        // }

        // return output;
    }
}
