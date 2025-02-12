use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

use crate::board::square::Square;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(Clone, Copy, Debug, Default)]
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
    fn new(color: Color) -> Self;
    fn move_to(&self, square: Square);
    fn color(&self) -> Color;
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
}
