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
    fn new(position: Position, color: Color) -> Self where Self: Sized;
    fn color(&self) -> Color;
    fn position(&self) -> Position;
    fn points(&self) -> i16;
    fn set_position(&mut self, position: Position);
    fn possible_moves(&self, board: &Board) -> HashSet<Move>;
    // fn with_has_moved(self) -> Self where Self : Sized;
    // fn with_en_passant_possible(self) -> Self where Self : Sized;
}
