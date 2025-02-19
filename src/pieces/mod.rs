use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

use crate::board::{square::Square, Board, COLUMNS, ROWS};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Color {
    #[default]
    WHITE,
    BLACK,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceKind {
    BISHOP(Bishop),
    KING(King),
    KNIGHT(Knight),
    PAWN(Pawn),
    QUEEN(Queen),
    ROOK(Rook),
}

pub trait Piece {
    fn new(index: usize) -> Self;
    fn move_to(&self, square: Square);
    fn possible_moves<'a>(&'a self, board: &'a Board, index: usize) -> Vec<&'a Square>;
    fn color(&self) -> Color;
}

impl PieceKind {
    pub fn new(index: usize) -> Option<Self> {
        if index / ROWS == 0 || index / ROWS == ROWS - 1 {
            match index % COLUMNS {
               0|7 => Some(Self::ROOK(Rook::new(index))),
               1|6 => Some(Self::KNIGHT(Knight::new(index))),
               2|5 => Some(Self::BISHOP(Bishop::new(index))),
               3   => Some(Self::QUEEN(Queen::new(index))),
               4   => Some(Self::KING(King::new(index))),
               _   => panic!("Should never happen: {index}")
            }
        } else if index / ROWS == 1 || index / ROWS == ROWS - 2 {
            Some(PieceKind::PAWN(Pawn::new(index)))
        } else {
            None
        }
    }

    pub fn color(&self) -> Color {
        match self {
            PieceKind::BISHOP(bishop) => bishop.color(),
            PieceKind::KING(king) => king.color(),
            PieceKind::KNIGHT(knight) => knight.color(),
            PieceKind::PAWN(pawn) => pawn.color(),
            PieceKind::QUEEN(queen) => queen.color(),
            PieceKind::ROOK(rook) => rook.color(),
        }
    }
}
