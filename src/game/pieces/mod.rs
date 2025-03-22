use std::collections::HashSet;

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::move_struct::Move;
use crate::game::board::position::Position;

pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod piece_kind;
pub(crate) mod queen;
pub(crate) mod rook;

pub trait Piece {
    fn color(&self) -> Color;
    fn position(&self) -> Position;
    fn points(&self) -> i8;
    fn set_position(&mut self, position: Position);
    fn possible_moves(&self, board: &Board) -> HashSet<Move>;
}
