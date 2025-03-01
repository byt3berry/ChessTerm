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

    pub fn position(&self) -> Position {
        match self {
            Self::BISHOP(bishop) => bishop.position(),
            Self::KING(king) => king.position(),
            Self::KNIGHT(knight) => knight.position(),
            Self::PAWN(pawn) => pawn.position(),
            Self::QUEEN(queen) => queen.position(),
            Self::ROOK(rook) => rook.position(),
        }
    }

    pub fn set_position(&mut self, position: Position) {
        match self {
            Self::BISHOP(bishop) => bishop.set_position(position),
            Self::KING(king) => king.set_position(position),
            Self::KNIGHT(knight) => knight.set_position(position),
            Self::PAWN(pawn) => pawn.set_position(position),
            Self::QUEEN(queen) => queen.set_position(position),
            Self::ROOK(rook) => rook.set_position(position),
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
