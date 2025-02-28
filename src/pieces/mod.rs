use std::collections::HashSet;

use color::Color;
use move_struct::Move;

use crate::board::Board;
use crate::board::position::Position;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;
pub mod color;
pub mod piece_kind;
pub mod move_struct;

pub trait Piece {
    fn new(position: Position, color: Color) -> Self;
    fn possible_moves(&self, board: &Board) -> HashSet<Move>;
    fn color(&self) -> Color;
    fn position(&self) -> &Position;
}
