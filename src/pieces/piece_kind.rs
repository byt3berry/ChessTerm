use std::collections::HashSet;

use crate::board::Board;
use crate::board::color::Color;
use crate::board::position::Position;

use super::bishop::Bishop;
use super::king::King;
use super::knight::Knight;
use super::pawn::Pawn;
use super::queen::Queen;
use super::rook::Rook;
use super::{Move, Piece};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PieceKind {
    Bishop(Bishop),
    King(King),
    Knight(Knight),
    Pawn(Pawn),
    Queen(Queen),
    Rook(Rook),
}

impl PieceKind {
    pub fn color(&self) -> Color {
        match self {
            Self::Bishop(bishop) => bishop.color(),
            Self::King(king) => king.color(),
            Self::Knight(knight) => knight.color(),
            Self::Pawn(pawn) => pawn.color(),
            Self::Queen(queen) => queen.color(),
            Self::Rook(rook) => rook.color(),
        }
    }

    pub fn position(&self) -> Position {
        match self {
            Self::Bishop(bishop) => bishop.position(),
            Self::King(king) => king.position(),
            Self::Knight(knight) => knight.position(),
            Self::Pawn(pawn) => pawn.position(),
            Self::Queen(queen) => queen.position(),
            Self::Rook(rook) => rook.position(),
        }
    }

    pub fn set_position(&mut self, position: Position) {
        match self {
            Self::Bishop(bishop) => bishop.set_position(position),
            Self::King(king) => king.set_position(position),
            Self::Knight(knight) => knight.set_position(position),
            Self::Pawn(pawn) => pawn.set_position(position),
            Self::Queen(queen) => queen.set_position(position),
            Self::Rook(rook) => rook.set_position(position),
        }
    }

    pub fn possible_moves(&self, board: &Board) -> HashSet<Move> {
        match self {
            Self::Bishop(bishop) => bishop.possible_moves(board),
            Self::King(king) => king.possible_moves(board),
            Self::Knight(knight) => knight.possible_moves(board),
            Self::Pawn(pawn) => pawn.possible_moves(board),
            Self::Queen(queen) => queen.possible_moves(board),
            Self::Rook(rook) => rook.possible_moves(board),
        }
    }
}
