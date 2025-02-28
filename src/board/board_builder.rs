use crate::board::position::Position;
use crate::board::square::Square;
use crate::board::{Board, COLUMNS, ROWS};
use crate::pieces::{Color, PieceKind};

use super::player::Player;

pub struct BoardBuilder {
    players: [Player; 2],
    board: [Square; ROWS*COLUMNS],
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self {
            players: [Player::new(Color::WHITE), Player::new(Color::BLACK)],
            board: [Square::default(); ROWS*COLUMNS],
        }
    }

    pub fn add(mut self, piece: PieceKind) -> Self {
        let position: &Position = piece.position();
        assert!(position.row() < ROWS, "position {position:?} is invalid");
        assert!(position.column() < COLUMNS, "position {position:?} is invalid");

        if let Some(index) = position.to_index() {
            self.board[index].set_piece(piece);
        }

        self
    }

    pub fn build(self) -> Board {
        Board::new(self.players, self.board)
    }
}
