use std::collections::HashSet;

use board_builder::BoardBuilder;
use player::Player;
use position::Position;
use square::Square;

use crate::pieces::Piece; 
use crate::pieces::bishop::Bishop;
use crate::pieces::color::Color;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::move_struct::{Move, MoveKind};
use crate::pieces::pawn::Pawn;
use crate::pieces::piece_kind::PieceKind;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;

pub mod board_builder;
pub mod player;
pub mod position;
pub mod square;

pub const ROWS: usize = 8;
pub const COLUMNS: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
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
            // Kings
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

    pub fn piece_mut_from_index(&mut self, index: usize) -> Option<&mut PieceKind> {
        if index >= ROWS * COLUMNS {
            return None;
        }

        if let Some(square) = self.board.get_mut(index) {
            return square.piece_mut();
        }

        None
    }

    pub fn piece_mut(&mut self, position: Position) -> Option<&mut PieceKind> {
        if let Some(square) = self.square_mut(position) {
            return square.piece_mut()
        }

        None
    }

    pub fn piece_unset(&mut self, position: Position) -> PieceKind {
        self
            .square_mut(position)
            .expect("The square in position {position:?} should exist")
            .piece_unset()
    }

    pub fn set_piece(&mut self, position: Position, piece: PieceKind) {
        self
            .square_mut(position)
            .expect("The square in position {position:?} should exist")
            .set_piece(piece);
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

    fn unset_all_en_passant(&mut self) {
        for index in 0..ROWS*COLUMNS {
            if let Some(PieceKind::PAWN(pawn)) = self.piece_mut_from_index(index) {
                pawn.unset_en_passant_possible();
            }
        }
    }

    pub fn make_move(&mut self, piece_move: Move) {
        let from: Position = piece_move.from();
        let to: Position = piece_move.to();

        match piece_move.kind() {
            MoveKind::Attack | MoveKind::PawnMove => {
                let mut piece: PieceKind = self.piece_unset(from);
                piece.set_position(to);
                self.set_piece(to, piece);
            }
            MoveKind::CastleKingSide => {
                let mut piece_king: PieceKind = self.piece_unset(from);
                let PieceKind::KING(king) = piece_king else {
                    panic!("The piece in position {from:?} should be a king");
                };

                let rook_position: Position = king.king_side_castling_rook_position();
                let mut piece_rook: PieceKind = self.piece_unset(rook_position);
                let PieceKind::ROOK(rook) = piece_rook else {
                    panic!("The piece in position {rook_position:?} should be a rook");
                };

                piece_king.set_position(to);
                self.set_piece(to, piece_king);

                let rook_final_position: Position = rook.king_side_castling_final_position();
                piece_rook.set_position(rook_final_position);
                self.set_piece(rook_final_position, piece_rook);
            }
            MoveKind::CastleQueenSide => {
                let mut piece_king: PieceKind = self.piece_unset(from);
                let PieceKind::KING(king) = piece_king else {
                    panic!("The piece in position {from:?} should be a king");
                };

                let rook_position: Position = king.queen_side_castling_rook_position();
                let mut piece_rook: PieceKind = self.piece_unset(rook_position);
                let PieceKind::ROOK(rook) = piece_rook else {
                    panic!("The piece in position {rook_position:?} should be a rook");
                };

                piece_king.set_position(to);
                self.set_piece(to, piece_king);

                let rook_final_position: Position = rook.queen_side_castling_final_position();
                piece_rook.set_position(rook_final_position);
                self.set_piece(rook_final_position, piece_rook);
                }
            MoveKind::EnPassant(attacked_position) => {
                let mut piece_attacker: PieceKind = self.piece_unset(from);
                let PieceKind::PAWN(_) = piece_attacker else {
                    panic!("The piece in position {from:?} should be a pawn");
                };

                let piece_attacked: PieceKind = self.piece_unset(attacked_position);
                let PieceKind::PAWN(_) = piece_attacked else {
                    panic!("The piece in position {from:?} should be a pawn");
                };

                piece_attacker.set_position(to);
                self.set_piece(to, piece_attacker);
            }
            MoveKind::Promotion => todo!(),
        }

        self.unset_all_en_passant();
    }

    pub fn simulate_move(&self, simulated_move: Move) -> Self {
        let mut simulated_board: Self = self.clone();
        simulated_board.make_move(simulated_move);

        simulated_board
    }

    fn king(&self, color: Color) -> Option<Position> {
        for index in 0..ROWS*COLUMNS {
            if let Some(PieceKind::KING(king)) = self.piece_from_index(index) {
                if king.color() == color {
                    return Some(king.position());
                }
            }
        }

        None
    }

    pub fn checked(&self, color: Color) -> bool {
        if let Some(position) = self.king(color) {
            self
                .player(color.other())
                .attacking()
                .iter()
                .any(|m| m.to() == position)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::board::board_builder::BoardBuilder;
    use crate::board::square::Square;
    use crate::pieces::king::King;
    use crate::pieces::Piece;
    use crate::pieces::bishop::Bishop;
    use crate::pieces::color::Color;
    use crate::pieces::move_struct::{Move, MoveKind};
    use crate::pieces::pawn::Pawn;
    use crate::pieces::piece_kind::PieceKind;
    use crate::pieces::rook::Rook;

    use super::Board;

    #[test]
    fn test_square() {
        let board: Board = Board::empty();
        let expected_piece: &Square = &Square::new(Color::WHITE, None);
        let expected: Option<&Square> = Some(expected_piece);

        let square: Option<&Square> = board.square((3isize, 3isize).into());

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
        let board: Board = BoardBuilder::new().build();
        let expected: Option<&Square> = None;

        let square: Option<&Square> = board.square((10isize, 12isize).into());

        assert_eq!(expected, square);
    }

    #[test]
    fn test_square_mut() {
        let mut board: Board = BoardBuilder::new().build();
        let expected_piece: &mut Square = &mut Square::new(Color::WHITE, None);
        let expected: Option<&mut Square> = Some(expected_piece);

        let square: Option<&mut Square> = board.square_mut((3isize, 3isize).into());

        assert_eq!(expected, square);
    }

    #[test]
    fn test_square_mut_invalid() {
        let mut board: Board = BoardBuilder::new().build();
        let expected: Option<&mut Square> = None;

        let square: Option<&mut Square> = board.square_mut((10isize, 12isize).into());

        assert_eq!(expected, square);
    }

    #[test]
    fn test_piece() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((3isize, 3isize).into(), Color::BLACK)))
            .build();
        let expected_piece: &PieceKind = &PieceKind::PAWN(Pawn::new((3isize, 3isize).into(), Color::BLACK));
        let expected: Option<&PieceKind> = Some(expected_piece);

        let piece: Option<&PieceKind> = board.piece((3isize, 3isize).into());

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_from_index() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((3isize, 3isize).into(), Color::BLACK)))
            .build();
        let expected_piece: &PieceKind = &PieceKind::PAWN(Pawn::new((3isize, 3isize).into(), Color::BLACK));
        let expected: Option<&PieceKind> = Some(expected_piece);

        let index: usize = 27usize;
        let piece: Option<&PieceKind> = board.piece_from_index(index);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_mut_from_index() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((3isize, 3isize).into(), Color::BLACK)))
            .build();
        let expected_piece: &mut PieceKind = &mut PieceKind::PAWN(Pawn::new((3isize, 3isize).into(), Color::BLACK));
        let expected: Option<&mut PieceKind> = Some(expected_piece);

        let index: usize = 27usize;
        let piece: Option<&mut PieceKind> = board.piece_mut_from_index(index);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_invalid() {
        let board: Board = BoardBuilder::new().build();
        let expected: Option<&PieceKind> = None;

        let piece: Option<&PieceKind> = board.piece((10isize, 12isize).into());

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_mut() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((3isize, 3isize).into(), Color::BLACK)))
            .build();
        let expected_piece: &mut PieceKind = &mut PieceKind::PAWN(Pawn::new((3isize, 3isize).into(), Color::BLACK));
        let expected: Option<&mut PieceKind> = Some(expected_piece);

        let piece: Option<&mut PieceKind> = board.piece_mut((3isize, 3isize).into());

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_mut_invalid() {
        let mut board: Board = BoardBuilder::new().build();
        let expected: Option<&mut PieceKind> = None;

        let piece: Option<&mut PieceKind> = board.piece_mut((10isize, 12isize).into());

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_set_attacking() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::ROOK(Rook::new((3isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::BISHOP(Bishop::new((6isize, 5isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((1isize, 1isize).into(), Color::WHITE)))
            .build();
        board.set_attacking(Color::BLACK);
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

        let attacking: &HashSet<Move> = board.player(Color::BLACK).attacking();

        assert_eq!(
            &expected,
            attacking,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&attacking),
            attacking.difference(&expected),
            );
    }

    #[test]
    #[should_panic]
    fn test_make_move_invalid() {
        let tested_move: Move = Move::new((6isize, 5isize).into(), (1isize, 2isize).into(), MoveKind::Attack);
        let mut board: Board = Board::empty();
        board.make_move(tested_move);
    }

    #[test]
    fn test_make_move_attack() {
        let tested_move: Move = Move::new((6isize, 5isize).into(), (1isize, 2isize).into(), MoveKind::Attack);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::BISHOP(Bishop::new((6isize, 5isize).into(), Color::BLACK)))
            .build();
        board.make_move(tested_move);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::BISHOP(Bishop::new((1isize, 2isize).into(), Color::BLACK)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_capture() {
        let tested_move: Move = Move::new((6isize, 5isize).into(), (1isize, 2isize).into(), MoveKind::Attack);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::BISHOP(Bishop::new((6isize, 5isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((1isize, 2isize).into(), Color::WHITE)))
            .build();
        board.make_move(tested_move);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::BISHOP(Bishop::new((1isize, 2isize).into(), Color::BLACK)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_pawn_move() {
        let tested_move: Move = Move::new((1isize, 2isize).into(), (2isize, 2isize).into(), MoveKind::PawnMove);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((1isize, 2isize).into(), Color::BLACK)))
            .build();
        board.make_move(tested_move);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((2isize, 2isize).into(), Color::BLACK)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_en_passant() {
        let tested_move: Move = Move::new((4isize, 2isize).into(), (5isize, 3isize).into(), MoveKind::EnPassant((4isize, 3isize).into()));
        let mut pawn: Pawn = Pawn::new((4isize, 3isize).into(), Color::WHITE);
        pawn.set_en_passant_possible();
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((4isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(pawn))
            .build();
        board.make_move(tested_move);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((5isize, 3isize).into(), Color::BLACK)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_ignore_en_passant() {
        let tested_move: Move = Move::new((4isize, 2isize).into(), (5isize, 2isize).into(), MoveKind::Attack);
        let mut pawn: Pawn = Pawn::new((4isize, 3isize).into(), Color::WHITE);
        pawn.set_en_passant_possible();
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((4isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(pawn))
            .build();
        board.make_move(tested_move);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new((5isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), Color::WHITE)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_queen_side_castle() {
        let tested_move: Move = Move::new((0isize, 4isize).into(), (0isize, 2isize).into(), MoveKind::CastleQueenSide);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), Color::BLACK)))
            .build();
        board.make_move(tested_move);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 3isize).into(), Color::BLACK)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_king_side_castle() {
        let tested_move: Move = Move::new((0isize, 4isize).into(), (0isize, 6isize).into(), MoveKind::CastleKingSide);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), Color::BLACK)))
            .build();
        board.make_move(tested_move);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 6isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 5isize).into(), Color::BLACK)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_checked() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((5isize, 4isize).into(), Color::WHITE)))
            .build();
        board.set_attacking(Color::WHITE);

        assert!(board.checked(Color::BLACK));
    }

    #[test]
    fn test_not_checked() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((5isize, 5isize).into(), Color::WHITE)))
            .build();
        board.set_attacking(Color::WHITE);

        assert!(!board.checked(Color::BLACK));
    }
}
