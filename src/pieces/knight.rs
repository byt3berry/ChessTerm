use crate::board::{Board, COLUMNS, ROWS};
use crate::board::square::Square;

use super::{Color, Piece};

#[derive(Clone, Copy, Debug)]
pub struct Knight {
    color: Color,
}

impl Piece for Knight {
    fn new(index: usize) -> Self {
        let color: Color = if index / ROWS <= 1 {
            Color::BLACK
        } else if index / ROWS >= 5 {
            Color::WHITE
        } else {
            panic!("Invalid index: {index}")
        };

        Self {
            color,
        }
    }

    fn move_to(&self, square: Square) {
        todo!()
    }

    fn color(&self) -> Color {
        self.color
    }

    fn possible_moves<'a>(&'a self, board: &'a Board, index: usize) -> Vec<&'a Square> {
        // index = row * ROWS + column
        let row: usize = index / ROWS;
        let column: usize = index % COLUMNS;
        let mut output: Vec<&Square> = Vec::new();

        if let Some(square) = board.get(row + 1, column + 2) {
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.push(square);
                }
            } else {
                output.push(square);
            }
        }

        if let Some(square) = board.get(row - 1, column + 2) {
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.push(square);
                }
            } else {
                output.push(square);
            }
        }

        if let Some(square) = board.get(row + 1, column - 2) {
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.push(square);
                }
            } else {
                output.push(square);
            }
        }

        if let Some(square) = board.get(row - 1, column - 2) {
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.push(square);
                }
            } else {
                output.push(square);
            }
        }

        if let Some(square) = board.get(row + 2, column + 1) {
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.push(square);
                }
            } else {
                output.push(square);
            }
        }

        if let Some(square) = board.get(row - 2, column + 1) {
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.push(square);
                }
            } else {
                output.push(square);
            }
        }

        if let Some(square) = board.get(row + 2, column - 1) {
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.push(square);
                }
            } else {
                output.push(square);
            }
        }

        if let Some(square) = board.get(row - 2, column - 1) {
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.push(square);
                }
            } else {
                output.push(square);
            }
        }

        return output;
    }
}
