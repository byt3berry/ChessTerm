use std::collections::HashSet;

use crate::board::Board;
use crate::board::color::Color;
use crate::board::move_struct::Move;
use crate::board::pin_kind::PinKind;
use crate::board::position::Position;

use piece_kind::PieceKind;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod piece_kind;
pub mod queen;
pub mod rook;

pub trait Piece {
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

            if let Some(PieceKind::King(pin_king)) = square.piece() {
                if pin_king.color() != self.color() {
                    king_found = true;
                    break;
                }
            }
        }

        king_found
    }

    fn pinned(&self, board: &Board) -> bool {
        board
            .player(self.color().other())
            .attacking()
            .iter()
            .filter(|m| m.to() == self.position())
            .any(|m| m.pin().is_some())
    }

    fn pinned_but_movable(&self, board: &Board, pin_kind: PinKind) -> bool {
        board
            .player(self.color().other())
            .attacking()
            .iter()
            .filter(|m| m.to() == self.position())
            .all(|m| m.pin().is_none_or(|p| p == pin_kind))
    }
}
