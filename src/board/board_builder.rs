use crate::pieces::{Color, PieceKind};
use crate::player::Player;
use super::position::Position;
use super::square::Square;
use super::{Board, COLUMNS, ROWS};

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
        assert!(position.row() < ROWS);
        assert!(position.column() < COLUMNS);

        self.board[position.to_index()].set_piece(piece);
        self
    }

    pub fn build(self) -> Board {
        Board::new(self.players, self.board)
    }
}
