use std::collections::HashSet;

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::position::Position;

use super::bishop::Bishop;
use super::king::King;
use super::knight::Knight;
use super::pawn::Pawn;
use super::queen::Queen;
use super::rook::Rook;
use super::{Move, Piece};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PieceKind {
    Bishop(Bishop),
    King(King),
    Knight(Knight),
    Pawn(Pawn),
    Queen(Queen),
    Rook(Rook),
}

impl Piece for PieceKind {
    fn new(_: Position, _: Color) -> Self {
        panic!("Type \"PieceKind\" can't be instantiated with new");
    }

    fn possible_moves(&self, board: &Board) -> HashSet<Move> {
        match self {
            Self::Bishop(bishop) => bishop.possible_moves(board),
            Self::King(king) => king.possible_moves(board),
            Self::Knight(knight) => knight.possible_moves(board),
            Self::Pawn(pawn) => pawn.possible_moves(board),
            Self::Queen(queen) => queen.possible_moves(board),
            Self::Rook(rook) => rook.possible_moves(board),
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Bishop(bishop) => bishop.color(),
            Self::King(king) => king.color(),
            Self::Knight(knight) => knight.color(),
            Self::Pawn(pawn) => pawn.color(),
            Self::Queen(queen) => queen.color(),
            Self::Rook(rook) => rook.color(),
        }
    }

    fn position(&self) -> Position {
        match self {
            Self::Bishop(bishop) => bishop.position(),
            Self::King(king) => king.position(),
            Self::Knight(knight) => knight.position(),
            Self::Pawn(pawn) => pawn.position(),
            Self::Queen(queen) => queen.position(),
            Self::Rook(rook) => rook.position(),
        }
    }

    fn set_position(&mut self, position: Position) {
        match self {
            Self::Bishop(bishop) => bishop.set_position(position),
            Self::King(king) => king.set_position(position),
            Self::Knight(knight) => knight.set_position(position),
            Self::Pawn(pawn) => pawn.set_position(position),
            Self::Queen(queen) => queen.set_position(position),
            Self::Rook(rook) => rook.set_position(position),
        }
    }

    fn find_king(&self, board: &Board, position: Position, offset: (isize, isize)) -> bool {
        match self {
            Self::Bishop(bishop) => bishop.find_king(board, position, offset),
            Self::King(king) => king.find_king(board, position, offset),
            Self::Knight(knight) => knight.find_king(board, position, offset),
            Self::Pawn(pawn) => pawn.find_king(board, position, offset),
            Self::Queen(queen) => queen.find_king(board, position, offset),
            Self::Rook(rook) => rook.find_king(board, position, offset),
        }
    }
}
