use std::collections::HashSet;

use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

use crate::board::position::Position;
use crate::board::Board;

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

pub enum MoveKind {
    Capture,
    Castle,
    EnPassant,
    Move,
}

pub struct Move {
    from: Position,
    to: Position,
    move_kind: MoveKind,
}

pub trait Piece {
    fn new(position: Position, color: Color) -> Self;
    fn possible_moves(&self, board: &Board, position: &Position) -> HashSet<Move>;
    fn color(&self) -> Color;
    fn position(&self) -> &Position;
}

impl PieceKind {
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

    pub fn position(&self) -> &Position {
        match self {
            PieceKind::BISHOP(bishop) => bishop.position(),
            PieceKind::KING(king) => king.position(),
            PieceKind::KNIGHT(knight) => knight.position(),
            PieceKind::PAWN(pawn) => pawn.position(),
            PieceKind::QUEEN(queen) => queen.position(),
            PieceKind::ROOK(rook) => rook.position(),
        }
    }
}
