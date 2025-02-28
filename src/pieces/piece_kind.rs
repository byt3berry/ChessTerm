use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;

use super::bishop::Bishop;
use super::color::Color;
use super::king::King;
use super::knight::Knight;
use super::pawn::Pawn;
use super::queen::Queen;
use super::rook::Rook;
use super::{Move, Piece};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PieceKind {
    BISHOP(Bishop),
    KING(King),
    KNIGHT(Knight),
    PAWN(Pawn),
    QUEEN(Queen),
    ROOK(Rook),
}

impl PieceKind {
    pub fn color(&self) -> Color {
        match self {
            Self::BISHOP(bishop) => bishop.color(),
            Self::KING(king) => king.color(),
            Self::KNIGHT(knight) => knight.color(),
            Self::PAWN(pawn) => pawn.color(),
            Self::QUEEN(queen) => queen.color(),
            Self::ROOK(rook) => rook.color(),
        }
    }

    pub fn position(&self) -> &Position {
        match self {
            Self::BISHOP(bishop) => bishop.position(),
            Self::KING(king) => king.position(),
            Self::KNIGHT(knight) => knight.position(),
            Self::PAWN(pawn) => pawn.position(),
            Self::QUEEN(queen) => queen.position(),
            Self::ROOK(rook) => rook.position(),
        }
    }

    pub fn possible_moves(&self, board: &Board) -> HashSet<Move> {
        match self {
            Self::BISHOP(bishop) => bishop.possible_moves(board),
            Self::KING(king) => king.possible_moves(board),
            Self::KNIGHT(knight) => knight.possible_moves(board),
            Self::PAWN(pawn) => pawn.possible_moves(board),
            Self::QUEEN(queen) => queen.possible_moves(board),
            Self::ROOK(rook) => rook.possible_moves(board),
        }
    }
}
