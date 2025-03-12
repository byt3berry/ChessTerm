use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use super::Piece;
use super::bishop::Bishop;
use super::king::King;
use super::knight::Knight;
use super::pawn::Pawn;
use super::queen::Queen;
use super::rook::Rook;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PieceKind {
    Bishop(Bishop),
    King(King),
    Knight(Knight),
    Pawn(Pawn),
    Queen(Queen),
    Rook(Rook),
}

impl Deref for PieceKind {
    type Target = dyn Piece;

    fn deref(&self) -> &Self::Target {
        match self {
            PieceKind::Bishop(bishop) => bishop,
            PieceKind::King(king) => king,
            PieceKind::Knight(knight) => knight,
            PieceKind::Pawn(pawn) => pawn,
            PieceKind::Queen(queen) => queen,
            PieceKind::Rook(rook) => rook,
        }
    }
}

impl DerefMut for PieceKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            PieceKind::Bishop(bishop) => bishop,
            PieceKind::King(king) => king,
            PieceKind::Knight(knight) => knight,
            PieceKind::Pawn(pawn) => pawn,
            PieceKind::Queen(queen) => queen,
            PieceKind::Rook(rook) => rook,
        }
    }
}
