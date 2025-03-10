use std::collections::HashSet;
use std::fmt::Debug;

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::move_struct::Move;
use crate::game::board::position::Position;

use piece_kind::PieceKind;

pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod piece_kind;
pub(crate) mod queen;
pub(crate) mod rook;

pub trait Piece 
where Self: Debug + Sized {
    fn new(position: Position, color: Color) -> Self;
    fn possible_moves(&self, board: &Board) -> HashSet<Move>;
    fn color(&self) -> Color;
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);

    fn find_king(&self, board: &Board, mut position: Position, offset: (isize, isize)) -> bool {
        let mut king_found: bool = false;

        loop {
            position = position + offset;
            let Some(square) = board.square(position) else {
                break;
            };

            if let Some(PieceKind::King(pin_king)) = square.piece(Color::Any) {
                if pin_king.color() != self.color() {
                    king_found = true;
                    break;
                }
            }
        }

        king_found
    }
}
