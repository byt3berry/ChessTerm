use board_builder::BoardBuilder;
use position::Position;
use square::Square;

use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
use crate::pieces::{Color, Piece, PieceKind};
use crate::player::Player;

pub mod board_builder;
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
            .add(PieceKind::ROOK(Rook::new((0usize, 0usize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0usize, 7usize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((7usize, 0usize).into(), Color::WHITE)))
            .add(PieceKind::ROOK(Rook::new((7usize, 7usize).into(), Color::WHITE)))
            // Knights
            .add(PieceKind::KNIGHT(Knight::new((0usize, 1usize).into(), Color::BLACK)))
            .add(PieceKind::KNIGHT(Knight::new((0usize, 6usize).into(), Color::BLACK)))
            .add(PieceKind::KNIGHT(Knight::new((7usize, 1usize).into(), Color::WHITE)))
            .add(PieceKind::KNIGHT(Knight::new((7usize, 6usize).into(), Color::WHITE)))
            // Bishops
            .add(PieceKind::BISHOP(Bishop::new((0usize, 2usize).into(), Color::BLACK)))
            .add(PieceKind::BISHOP(Bishop::new((0usize, 5usize).into(), Color::BLACK)))
            .add(PieceKind::BISHOP(Bishop::new((7usize, 2usize).into(), Color::WHITE)))
            .add(PieceKind::BISHOP(Bishop::new((7usize, 5usize).into(), Color::WHITE)))
            // Queens
            .add(PieceKind::QUEEN(Queen::new((0usize, 3usize).into(), Color::BLACK)))
            .add(PieceKind::QUEEN(Queen::new((7usize, 3usize).into(), Color::WHITE)))
            // Queens
            .add(PieceKind::KING(King::new((0usize, 4usize).into(), Color::BLACK)))
            .add(PieceKind::KING(King::new((7usize, 4usize).into(), Color::WHITE)));

        // Pawns
        for j in 0..COLUMNS {
            board_builder = board_builder.add(PieceKind::PAWN(Pawn::new((1usize, j).into(), Color::BLACK)));
        }
        for j in 0..COLUMNS {
            board_builder = board_builder.add(PieceKind::PAWN(Pawn::new((6usize, j).into(), Color::WHITE)));
        }

        board_builder.build()
    }

    pub fn empty() -> Self {
        BoardBuilder::new().build()
    }

    pub const fn new(players: [Player; 2], board: [Square; ROWS*COLUMNS]) -> Self {
        Self {
            players,
            board,
        }
    }

    pub fn square(&self, position: Position) -> Option<&Square> {
        if let Some(index) = position.to_index() {
            return self.board.get(index);
        }

        None
    }

    pub fn square_mut(&mut self, position: Position) -> Option<&mut Square> {
        if let Some(index) = position.to_index() {
            return self.board.get_mut(index);
        }

        None
    }

    pub fn piece(&self, position: Position) -> Option<&PieceKind> {
        if let Some(square) = self.square(position) {
            return square.piece()
        }

        None
    }

    pub fn piece_mut(&mut self, position: Position) -> Option<&mut PieceKind> {
        if let Some(square) = self.square_mut(position) {
            return square.piece_mut()
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::board::board_builder::BoardBuilder;
    use crate::board::position::Position;
    use crate::board::square::Square;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::{Color, Piece, PieceKind};

    use super::Board;

    #[test]
    fn test_square() {
        let position: Position = (3usize, 3usize).into();
        let board: Board = Board::empty();
        let expected_piece: &Square = &Square::new(Color::WHITE, None);
        let expected: Option<&Square> = Some(expected_piece);

        let square: Option<&Square> = board.square(position);

        assert_eq!(expected, square);
    }

    #[test]
    fn test_square_invalid() {
        let position: Position = (10usize, 12usize).into();
        let board: Board = BoardBuilder::new().build();
        let expected: Option<&Square> = None;

        let square: Option<&Square> = board.square(position);

        assert_eq!(expected, square);
    }

    #[test]
    fn test_square_mut() {
        let position: Position = (3usize, 3usize).into();
        let mut board: Board = BoardBuilder::new().build();
        let expected_piece: &mut Square = &mut Square::new(Color::WHITE, None);
        let expected: Option<&mut Square> = Some(expected_piece);

        let square: Option<&mut Square> = board.square_mut(position);

        assert_eq!(expected, square);
    }

    #[test]
    fn test_square_mut_invalid() {
        let position: Position = (10usize, 12usize).into();
        let mut board: Board = BoardBuilder::new().build();
        let expected: Option<&mut Square> = None;

        let square: Option<&mut Square> = board.square_mut(position);

        assert_eq!(expected, square);
    }

    #[test]
    fn test_piece() {
        let position: Position = (3usize, 3usize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new(position, color)))
            .build();
        let expected_piece: &PieceKind = &PieceKind::PAWN(Pawn::new(position, color));
        let expected: Option<&PieceKind> = Some(expected_piece);

        let piece: Option<&PieceKind> = board.piece(position);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_invalid() {
        let position: Position = (10usize, 12usize).into();
        let board: Board = BoardBuilder::new().build();
        let expected: Option<&PieceKind> = None;

        let piece: Option<&PieceKind> = board.piece(position);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_mut() {
        let position: Position = (3usize, 3usize).into();
        let color: Color = Color::BLACK;
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new(position, color)))
            .build();
        let expected_piece: &mut PieceKind = &mut PieceKind::PAWN(Pawn::new(position, color));
        let expected: Option<&mut PieceKind> = Some(expected_piece);

        let piece: Option<&mut PieceKind> = board.piece_mut(position);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_mut_invalid() {
        let position: Position = (10usize, 12usize).into();
        let mut board: Board = BoardBuilder::new().build();
        let expected: Option<&mut PieceKind> = None;

        let piece: Option<&mut PieceKind> = board.piece_mut(position);

        assert_eq!(expected, piece);
    }
}
