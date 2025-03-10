use crate::game::pieces::piece_kind::PieceKind;

use super::color::Color;
use super::player::Player;
use super::position::Position;
use super::square::Square;
use super::{Board, COLUMNS, ROWS};

pub struct BoardBuilder {
    players: [Player; 2],
    board: [Square; ROWS*COLUMNS],
}

impl BoardBuilder {
    pub(crate) fn new() -> Self {
        Self {
            players: [Player::new(Color::White), Player::new(Color::Black)],
            board: [Square::default(); ROWS*COLUMNS],
        }
    }

    pub(crate) fn add(mut self, piece: PieceKind) -> Self {
        let position: Position = piece.position();
        assert!(position.row() < ROWS, "position {position:?} is invalid");
        assert!(position.column() < COLUMNS, "position {position:?} is invalid");

        if let Some(index) = position.to_index() {
            self.board[index].set_piece(piece);
        }

        self
    }

    pub(crate) fn build(self) -> Board {
        Board::new(self.players, self.board)
    }
}
