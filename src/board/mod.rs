use std::collections::HashSet;

use board_builder::BoardBuilder;
use player::Player;
use position::Position;
use square::Square;

use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
use crate::pieces::{Color, Move, Piece, PieceKind};

pub mod board_builder;
pub mod player;
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
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((7isize, 0isize).into(), Color::WHITE)))
            .add(PieceKind::ROOK(Rook::new((7isize, 7isize).into(), Color::WHITE)))
            // Knights
            .add(PieceKind::KNIGHT(Knight::new((0isize, 1isize).into(), Color::BLACK)))
            .add(PieceKind::KNIGHT(Knight::new((0isize, 6isize).into(), Color::BLACK)))
            .add(PieceKind::KNIGHT(Knight::new((7isize, 1isize).into(), Color::WHITE)))
            .add(PieceKind::KNIGHT(Knight::new((7isize, 6isize).into(), Color::WHITE)))
            // Bishops
            .add(PieceKind::BISHOP(Bishop::new((0isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::BISHOP(Bishop::new((0isize, 5isize).into(), Color::BLACK)))
            .add(PieceKind::BISHOP(Bishop::new((7isize, 2isize).into(), Color::WHITE)))
            .add(PieceKind::BISHOP(Bishop::new((7isize, 5isize).into(), Color::WHITE)))
            // Queens
            .add(PieceKind::QUEEN(Queen::new((0isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::QUEEN(Queen::new((7isize, 3isize).into(), Color::WHITE)))
            // Queens
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::KING(King::new((7isize, 4isize).into(), Color::WHITE)));

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

    pub const fn player(&self, color: Color) -> &Player {
        match color {
            Color::WHITE => &self.players[0],
            Color::BLACK => &self.players[1],
        }
    }

    pub const fn player_mut(&mut self, color: Color) -> &mut Player {
        match color {
            Color::WHITE => &mut self.players[0],
            Color::BLACK => &mut self.players[1],
        }
    }

    pub fn square(&self, position: Position) -> Option<&Square> {
        if let Some(index) = position.to_index() {
            return self.board.get(index);
        }

        None
    }

    pub fn square_from_index(&self, index: usize) -> Option<&Square> {
        if index >= ROWS * COLUMNS {
            return None;
        }

        self.board.get(index)
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

    pub fn piece_from_index(&self, index: usize) -> Option<&PieceKind> {
        if index >= ROWS * COLUMNS {
            return None;
        }

        if let Some(square) = self.board.get(index) {
            return square.piece();
        }

        None
    }

    pub fn piece_mut(&mut self, position: Position) -> Option<&mut PieceKind> {
        if let Some(square) = self.square_mut(position) {
            return square.piece_mut()
        }

        None
    }

    pub fn set_attacking(&mut self, color: Color) {
        let mut attacking: HashSet<Move> = HashSet::new();

        for index in 0..ROWS*COLUMNS {
            if let Some(piece) = self.piece_from_index(index) {
                if piece.color() == color {
                    attacking.extend(piece.possible_moves(self));
                }
            }
        }

        self.player_mut(color).set_attacking(attacking);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::board::board_builder::BoardBuilder;
    use crate::board::position::Position;
    use crate::board::square::Square;
    use crate::pieces::bishop::Bishop;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::rook::Rook;
    use crate::pieces::{Color, Move, MoveKind, Piece, PieceKind};

    use super::Board;

    #[test]
    fn test_square() {
        let position: Position = (3isize, 3isize).into();
        let board: Board = Board::empty();
        let expected_piece: &Square = &Square::new(Color::WHITE, None);
        let expected: Option<&Square> = Some(expected_piece);

        let square: Option<&Square> = board.square(position);

        assert_eq!(expected, square);
    }

    #[test]
    fn test_square_from_index() {
        let index: usize = 27usize;
        let board: Board = Board::empty();
        let expected_piece: &Square = &Square::new(Color::WHITE, None);
        let expected: Option<&Square> = Some(expected_piece);

        let square: Option<&Square> = board.square_from_index(index);

        assert_eq!(expected, square);
    }

    #[test]
    fn test_square_invalid() {
        let position: Position = (10isize, 12isize).into();
        let board: Board = BoardBuilder::new().build();
        let expected: Option<&Square> = None;

        let square: Option<&Square> = board.square(position);

        assert_eq!(expected, square);
    }

    #[test]
    fn test_square_mut() {
        let position: Position = (3isize, 3isize).into();
        let mut board: Board = BoardBuilder::new().build();
        let expected_piece: &mut Square = &mut Square::new(Color::WHITE, None);
        let expected: Option<&mut Square> = Some(expected_piece);

        let square: Option<&mut Square> = board.square_mut(position);

        assert_eq!(expected, square);
    }

    #[test]
    fn test_square_mut_invalid() {
        let position: Position = (10isize, 12isize).into();
        let mut board: Board = BoardBuilder::new().build();
        let expected: Option<&mut Square> = None;

        let square: Option<&mut Square> = board.square_mut(position);

        assert_eq!(expected, square);
    }

    #[test]
    fn test_piece() {
        let position: Position = (3isize, 3isize).into();
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
    fn test_piece_from_index() {
        let position: Position = (3isize, 3isize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new(position, color)))
            .build();
        let expected_piece: &PieceKind = &PieceKind::PAWN(Pawn::new(position, color));
        let expected: Option<&PieceKind> = Some(expected_piece);

        let index: usize = 27usize;
        let piece: Option<&PieceKind> = board.piece_from_index(index);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_invalid() {
        let position: Position = (10isize, 12isize).into();
        let board: Board = BoardBuilder::new().build();
        let expected: Option<&PieceKind> = None;

        let piece: Option<&PieceKind> = board.piece(position);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_mut() {
        let position: Position = (3isize, 3isize).into();
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
        let position: Position = (10isize, 12isize).into();
        let mut board: Board = BoardBuilder::new().build();
        let expected: Option<&mut PieceKind> = None;

        let piece: Option<&mut PieceKind> = board.piece_mut(position);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_set_attacking() {
        let color: Color = Color::BLACK;
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::ROOK(Rook::new((3isize, 3isize).into(), color)))
            .add(PieceKind::BISHOP(Bishop::new((6isize, 5isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((1isize, 1isize).into(), color.other())))
            .build();
        board.set_attacking(color);
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (0isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (5isize, 6isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 0isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 1isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 5isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 6isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 7isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (6isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (7isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (1isize, 0isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (2isize, 1isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (3isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (4isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (4isize, 7isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (5isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (5isize, 6isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (7isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((6isize, 5isize).into(), (7isize, 6isize).into(), MoveKind::Attack));

        let attacking: &HashSet<Move> = board.player(color).attacking();

        assert_eq!(
            &expected,
            attacking,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&attacking),
            attacking.difference(&expected),
            );
    }
}
