use std::collections::HashSet;

use board::Board;
use board::color::Color;
use board::move_struct::Move;
use board::position::Position;
use board::square::Square;

pub(super) mod pieces;
pub(super) mod board;

pub struct ChessEngine {
    board: Board,
    current_player: Color,
    possible_moves: Option<HashSet<Move>>,
}

impl ChessEngine {
    pub fn new() -> Self {
        Self {
            board: Board::init(),
            current_player: Color::White,
            possible_moves: None,
        }
    }

    pub fn square(&self, position: Position) -> Option<&Square> {
        self.board.square(position)
    }

    pub fn try_move(&mut self, from: Option<Position>, to: Option<Position>) -> bool {
        let Some(from) = from else {
            return false;
        };
        let Some(to) = to else {
            return false;
        };
        let Some(possible_moves) = self.possible_moves.as_ref() else {
            return false;
        };
        let Some(try_move) = possible_moves
            .iter()
            .find(|m| m.from() == from && m.to() == to) else {
                return false;
            };

        self.board.make_move(try_move, self.current_player);
        self.current_player = self.current_player.other();
        true
    }

    pub fn set_possible_moves(&mut self, position: Option<Position>) {
        let Some(position) = position else {
            return;
        };

        self.board.set_attacking(self.current_player.other());
        if let Some(piece) = self.board.piece(position) {
            if piece.color() == self.current_player {
                self.possible_moves = Some(piece.possible_moves(&self.board));
                return;
            }
        }

        self.possible_moves = None;
    }

    pub fn possible_moves(&self, position: Option<Position>) -> Option<HashSet<Position>> {
        let possible_moves: &HashSet<Move> = self.possible_moves.as_ref()?;
        let position = position?;

        if let Some(piece) = self.board.piece(position) {
            if piece.color() == self.current_player {
                return Some(possible_moves
                    .iter()
                    .map(|m| m.to())
                    .collect());
            }
        }

        None
    }
}
