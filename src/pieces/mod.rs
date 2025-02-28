use std::collections::HashSet;

use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

use crate::board::Board;
use crate::board::position::Position;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Color {
    #[default]
    WHITE,
    BLACK,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PieceKind {
    BISHOP(Bishop),
    KING(King),
    KNIGHT(Knight),
    PAWN(Pawn),
    QUEEN(Queen),
    ROOK(Rook),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveKind {
    Attack,
    CastleKingSide,
    CastleQueenSide,
    EnPassant,
    PawnMove,
    Promotion,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Move {
    from: Position,
    to: Position,
    kind: MoveKind,
}

pub trait Piece {
    fn new(position: Position, color: Color) -> Self;
    fn possible_moves(&self, board: &Board) -> HashSet<Move>;
    fn color(&self) -> Color;
    fn position(&self) -> &Position;
}

impl Color {
    pub const fn other(self) -> Self {
        match self {
            Self::WHITE => Self::BLACK,
            Self::BLACK => Self::WHITE,
        }
    }
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

impl Move {
    pub const fn new(from: Position, to: Position, kind: MoveKind) -> Self {
        Self {
            from,
            to,
            kind,
        }
    }

    pub const fn kind(&self) -> MoveKind {
        self.kind
    }

    pub const fn from(&self) -> &Position {
        &self.from
    }

    pub const fn to(&self) -> &Position {
        &self.to
    }
}
