use board_builder::BoardBuilder;
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
use crate::pieces::{Color, Piece, PieceKind};
use crate::player::Player;
use position::Position;
use square::Square;

mod board_builder;
pub mod position;
pub mod square;

pub const ROWS: usize = 8;
pub const COLUMNS: usize = 8;

#[derive(Debug)]
pub struct Board {
    players: [Player; 2],
    board: [Square; ROWS*COLUMNS],
}

impl Board {
    pub fn init() -> Self {
        let mut board_builder = BoardBuilder::new()
            // Rooks
            .add(PieceKind::ROOK(Rook::new((0, 0).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0, COLUMNS-1).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((ROWS-1, 0).into(), Color::WHITE)))
            .add(PieceKind::ROOK(Rook::new((ROWS-1, COLUMNS-1).into(), Color::WHITE)))
            // Knights
            .add(PieceKind::KNIGHT(Knight::new((0, 1).into(), Color::BLACK)))
            .add(PieceKind::KNIGHT(Knight::new((0, COLUMNS-2).into(), Color::BLACK)))
            .add(PieceKind::KNIGHT(Knight::new((ROWS-1, 1).into(), Color::WHITE)))
            .add(PieceKind::KNIGHT(Knight::new((ROWS-1, COLUMNS-2).into(), Color::WHITE)))
            // Bishops
            .add(PieceKind::BISHOP(Bishop::new((0, 2).into(), Color::BLACK)))
            .add(PieceKind::BISHOP(Bishop::new((0, COLUMNS-3).into(), Color::BLACK)))
            .add(PieceKind::BISHOP(Bishop::new((ROWS-1, 2).into(), Color::WHITE)))
            .add(PieceKind::BISHOP(Bishop::new((ROWS-1, COLUMNS-3).into(), Color::WHITE)))
            // Queens
            .add(PieceKind::QUEEN(Queen::new((0, 3).into(), Color::BLACK)))
            .add(PieceKind::QUEEN(Queen::new((ROWS-1, 3).into(), Color::WHITE)))
            // Queens
            .add(PieceKind::KING(King::new((0, COLUMNS-4).into(), Color::BLACK)))
            .add(PieceKind::KING(King::new((ROWS-1, COLUMNS-4).into(), Color::WHITE)));

        // Pawns
        for j in 0..COLUMNS {
            board_builder = board_builder.add(PieceKind::PAWN(Pawn::new((1, j).into(), Color::BLACK)));
        }
        for j in 0..COLUMNS {
            board_builder = board_builder.add(PieceKind::PAWN(Pawn::new((ROWS-2, j).into(), Color::WHITE)));
        }

        board_builder.build()
    }

    pub fn empty() -> Self {
        BoardBuilder::new().build()
    }

    pub fn new(players: [Player; 2], board: [Square; ROWS*COLUMNS]) -> Self {
        Self {
            players,
            board,
        }
    }

    pub fn get(&self, position: &Position) -> Option<&Square> {
        self.board.get(position.to_index())
    }
}
