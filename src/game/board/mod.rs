use std::collections::HashSet;

use board_builder::BoardBuilder;
use color::Color;
use move_kind::MoveKind;
use move_struct::Move;
use player::Player;
use position::Position;
use square::Square;

use crate::game::pieces::Piece; 
use crate::game::pieces::bishop::Bishop;
use crate::game::pieces::king::King;
use crate::game::pieces::knight::Knight;
use crate::game::pieces::pawn::Pawn;
use crate::game::pieces::piece_kind::PieceKind;
use crate::game::pieces::queen::Queen;
use crate::game::pieces::rook::Rook;

pub mod board_builder;
pub(crate) mod color;
pub(super) mod move_struct;
pub(super) mod player;
pub(crate) mod position;
pub(crate) mod square;
pub mod move_kind;

pub(crate) const ROWS: usize = 8;
pub(crate) const COLUMNS: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Board {
    players: [Player; 2],
    board: [Square; ROWS*COLUMNS],
}

impl Board {
    pub(crate) fn init() -> Self {
        let mut board_builder = BoardBuilder::new()
            // Rooks
            .add(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((7isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((7isize, 7isize).into(), Color::White)))
            // Knights
            .add(PieceKind::Knight(Knight::new((0isize, 1isize).into(), Color::Black)))
            .add(PieceKind::Knight(Knight::new((0isize, 6isize).into(), Color::Black)))
            .add(PieceKind::Knight(Knight::new((7isize, 1isize).into(), Color::White)))
            .add(PieceKind::Knight(Knight::new((7isize, 6isize).into(), Color::White)))
            // Bishops
            .add(PieceKind::Bishop(Bishop::new((0isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((0isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((7isize, 2isize).into(), Color::White)))
            .add(PieceKind::Bishop(Bishop::new((7isize, 5isize).into(), Color::White)))
            // Queens
            .add(PieceKind::Queen(Queen::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Queen(Queen::new((7isize, 3isize).into(), Color::White)))
            // Kings
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((7isize, 4isize).into(), Color::White)));

        // Pawns
        for j in 0..COLUMNS {
            board_builder = board_builder.add(PieceKind::Pawn(Pawn::new((1usize, j).into(), Color::Black)));
        }
        for j in 0..COLUMNS {
            board_builder = board_builder.add(PieceKind::Pawn(Pawn::new((6usize, j).into(), Color::White)));
        }

        board_builder.build()
    }

    const fn new(players: [Player; 2], board: [Square; ROWS*COLUMNS]) -> Self {
        Self {
            players,
            board,
        }
    }

    pub(super) const fn player(&self, color: Color) -> &Player {
        match color {
            Color::White => &self.players[0],
            Color::Black => &self.players[1],
            Color::Any => panic!("No player can be associated with color \"Any\""),
        }
    }

    const fn player_mut(&mut self, color: Color) -> &mut Player {
        match color {
            Color::White => &mut self.players[0],
            Color::Black => &mut self.players[1],
            Color::Any => panic!("No player can be associated with color \"Any\""),
        }
    }

    pub(crate) fn square(&self, position: Position) -> Option<&Square> {
        self.board.get(position.to_index()?)
    }

    fn square_mut(&mut self, position: Position) -> Option<&mut Square> {
        self.board.get_mut(position.to_index()?)
    }

    pub(super) fn piece(&self, position: Position, color: Color) -> Option<&PieceKind> {
        self.square(position)?.piece(color)
    }

    fn piece_unset(&mut self, position: Position) -> PieceKind {
        self
            .square_mut(position)
            .expect("The square in position {position:?} should exist")
            .piece_unset()
    }

    fn set_piece(&mut self, position: Position, piece: PieceKind) {
        self
            .square_mut(position)
            .expect("The square in position {position:?} should exist")
            .set_piece(piece);
    }

    pub(super) fn set_possible_moves(&mut self, color: Color) {
        let possible_moves: HashSet<Move> = self
            .board
            .iter()
            .filter_map(|square| square.piece(color))
            .flat_map(|piece| piece.possible_moves(self))
            .collect();

        self.player_mut(color).set_possible_moves(possible_moves);
    }

    fn unset_all_en_passant(&mut self, color: Color) {
        self.board
            .iter_mut()
            .filter_map(|square| square.piece_mut(color))
            .filter_map(|piece| if let PieceKind::Pawn(pawn) = piece { Some(pawn) } else { None })
            .for_each(|pawn| pawn.unset_en_passant_possible());
        }

    pub fn make_move(&mut self, piece_move: &Move, color: Color) {
        let from: Position = piece_move.from();
        let to: Position = piece_move.to();

        match piece_move.kind() {
            MoveKind::Attack => {
                let mut piece: PieceKind = self.piece_unset(from);

                if let PieceKind::Rook(ref mut rook) = piece {
                    rook.set_has_moved();
                } else if let PieceKind::King(ref mut king) = piece {
                    king.set_has_moved();
                };

                piece.set_position(to);
                self.set_piece(to, piece);
            }
            MoveKind::PawnSimpleMove => {
                let mut piece: PieceKind = self.piece_unset(from);
                piece.set_position(to);

                let PieceKind::Pawn(ref mut pawn) = piece else {
                    panic!("Only pawn can make SimplePawnMove");
                };

                pawn.set_has_moved();
                self.set_piece(to, piece);
            }
            MoveKind::PawnDoubleMove => {
                let mut piece: PieceKind = self.piece_unset(from);
                piece.set_position(to);

                let PieceKind::Pawn(ref mut pawn) = piece else {
                    panic!("Only pawn can make SimplePawnMove");
                };

                pawn.set_has_moved();
                pawn.set_en_passant_possible();
                self.set_piece(to, piece);
            }
            MoveKind::CastleKingSide => {
                let mut piece_king: PieceKind = self.piece_unset(from);
                let PieceKind::King(ref mut king) = piece_king else {
                    panic!("The piece in position {from:?} should be a king");
                };
                king.set_has_moved();

                let rook_position: Position = king.king_side_castling_rook_position();
                let mut piece_rook: PieceKind = self.piece_unset(rook_position);
                let PieceKind::Rook(ref mut rook) = piece_rook else {
                    panic!("The piece in position {rook_position:?} should be a rook");
                };
                rook.set_has_moved();

                piece_king.set_position(to);
                self.set_piece(to, piece_king);

                let rook_final_position: Position = rook.king_side_castling_final_position();
                piece_rook.set_position(rook_final_position);
                self.set_piece(rook_final_position, piece_rook);
            }
            MoveKind::CastleQueenSide => {
                let mut piece_king: PieceKind = self.piece_unset(from);
                let PieceKind::King(ref mut king) = piece_king else {
                    panic!("The piece in position {from:?} should be a king");
                };
                king.set_has_moved();

                let rook_position: Position = king.queen_side_castling_rook_position();
                let mut piece_rook: PieceKind = self.piece_unset(rook_position);
                let PieceKind::Rook(ref mut rook) = piece_rook else {
                    panic!("The piece in position {rook_position:?} should be a rook");
                };
                rook.set_has_moved();

                piece_king.set_position(to);
                self.set_piece(to, piece_king);

                let rook_final_position: Position = rook.queen_side_castling_final_position();
                piece_rook.set_position(rook_final_position);
                self.set_piece(rook_final_position, piece_rook);
                }
            MoveKind::EnPassant(attacked_position) => {
                let mut piece_attacker: PieceKind = self.piece_unset(from);
                let PieceKind::Pawn(_) = piece_attacker else {
                    panic!("The piece in position {from:?} should be a pawn");
                };

                let piece_attacked: PieceKind = self.piece_unset(attacked_position);
                let PieceKind::Pawn(_) = piece_attacked else {
                    panic!("The piece in position {attacked_position:?} should be a pawn");
                };

                piece_attacker.set_position(to);
                self.set_piece(to, piece_attacker);
            }
            MoveKind::Promotion => todo!(),
        }

        self.unset_all_en_passant(color.other());
    }

    pub fn simulate_move(&self, simulated_move: &Move, color: Color) -> Self {
        let mut simulated_board: Self = self.clone();
        simulated_board.make_move(simulated_move, color);

        simulated_board
    }

    pub fn king(&self, color: Color) -> Option<Position> {
        self
            .board
            .iter()
            .filter_map(|square| square.piece(color))
            .filter(|piece| matches!(piece, PieceKind::King(_)))
            .map(|piece| piece.position())
            .next()
    }

    pub fn possible_moves(&self, color: Color) -> &HashSet<Move> {
        self
            .player(color)
            .possible_moves()
    }

    pub fn checked(&self, color: Color) -> bool {
        if let Some(king_position) = self.king(color) {
            self
                .possible_moves(color.other())
                .iter()
                .any(|m| m.to() == king_position)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::game::board::board_builder::BoardBuilder;
    use crate::game::board::color::Color;
    use crate::game::board::move_kind::MoveKind;
    use crate::game::board::move_struct::Move;
    use crate::game::board::square::Square;
    use crate::game::pieces::Piece;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::king::King;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;
    use crate::game::pieces::rook::Rook;

    use super::Board;

    #[test]
    fn test_square() {
        let board: Board = BoardBuilder::new().build();
        let expected_piece: &Square = &Square::new(None);
        let expected: Option<&Square> = Some(expected_piece);

        let square: Option<&Square> = board.square((3isize, 3isize).into());

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
        let expected_piece: &mut Square = &mut Square::new(None);
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
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .build();
        let expected_piece: &PieceKind = &PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black));
        let expected: Option<&PieceKind> = Some(expected_piece);

        let piece: Option<&PieceKind> = board.piece((3isize, 3isize).into(), Color::Black);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_piece_invalid() {
        let board: Board = BoardBuilder::new().build();
        let expected: Option<&PieceKind> = None;

        let piece: Option<&PieceKind> = board.piece((10isize, 12isize).into(), Color::Black);

        assert_eq!(expected, piece);
    }

    #[test]
    fn test_set_attacking() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Rook(Rook::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 1isize).into(), Color::White)))
            .build();
        board.set_possible_moves(Color::Black);
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

        let possible_moves: &HashSet<Move> = board.possible_moves(Color::Black);

        assert_eq!(
            &expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    #[should_panic]
    fn test_make_move_invalid() {
        let tested_move: Move = Move::new((6isize, 5isize).into(), (1isize, 2isize).into(), MoveKind::Attack);
        let mut board: Board = BoardBuilder::new().build();
        board.make_move(&tested_move, Color::White);
    }

    #[test]
    fn test_make_move_attack() {
        let tested_move: Move = Move::new((6isize, 5isize).into(), (1isize, 2isize).into(), MoveKind::Attack);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Bishop(Bishop::new((6isize, 5isize).into(), Color::Black)))
            .build();
        board.make_move(&tested_move, Color::Black);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::Bishop(Bishop::new((1isize, 2isize).into(), Color::Black)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_capture() {
        let tested_move: Move = Move::new((6isize, 5isize).into(), (1isize, 2isize).into(), MoveKind::Attack);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Bishop(Bishop::new((6isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 2isize).into(), Color::White)))
            .build();
        board.make_move(&tested_move, Color::Black);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::Bishop(Bishop::new((1isize, 2isize).into(), Color::Black)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_pawn_move() {
        let tested_move: Move = Move::new((1isize, 2isize).into(), (2isize, 2isize).into(), MoveKind::PawnSimpleMove);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((1isize, 2isize).into(), Color::Black)))
            .build();
        board.make_move(&tested_move, Color::Black);

        let mut expected_pawn: Pawn = Pawn::new((2isize, 2isize).into(), Color::Black);
        expected_pawn.set_has_moved();
        let expected: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(expected_pawn))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_en_passant() {
        let tested_move: Move = Move::new((4isize, 2isize).into(), (5isize, 3isize).into(), MoveKind::EnPassant((4isize, 3isize).into()));
        let mut pawn: Pawn = Pawn::new((4isize, 3isize).into(), Color::White);
        pawn.set_en_passant_possible();
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(pawn))
            .build();
        board.make_move(&tested_move, Color::Black);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((5isize, 3isize).into(), Color::Black)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_ignore_en_passant() {
        let tested_move: Move = Move::new((4isize, 2isize).into(), (5isize, 2isize).into(), MoveKind::Attack);
        let mut pawn: Pawn = Pawn::new((4isize, 3isize).into(), Color::White);
        pawn.set_en_passant_possible();
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(pawn))
            .build();
        board.make_move(&tested_move, Color::Black);

        let expected: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((5isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::White)))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_queen_side_castle() {
        let tested_move: Move = Move::new((0isize, 4isize).into(), (0isize, 2isize).into(), MoveKind::CastleQueenSide);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .build();
        board.make_move(&tested_move, Color::Black);

        let mut expected_king: King = King::new((0isize, 2isize).into(), Color::Black);
        expected_king.set_has_moved();
        let mut expected_rook: Rook = Rook::new((0isize, 3isize).into(), Color::Black);
        expected_rook.set_has_moved();
        let expected: Board = BoardBuilder::new()
            .add(PieceKind::King(expected_king))
            .add(PieceKind::Rook(expected_rook))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_make_move_king_side_castle() {
        let tested_move: Move = Move::new((0isize, 4isize).into(), (0isize, 6isize).into(), MoveKind::CastleKingSide);
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .build();
        board.make_move(&tested_move, Color::Black);

        let mut expected_king: King = King::new((0isize, 6isize).into(), Color::Black);
        expected_king.set_has_moved();
        let mut expected_rook: Rook = Rook::new((0isize, 5isize).into(), Color::Black);
        expected_rook.set_has_moved();
        let expected: Board = BoardBuilder::new()
            .add(PieceKind::King(expected_king))
            .add(PieceKind::Rook(expected_rook))
            .build();

        assert_eq!(expected, board);
    }

    #[test]
    fn test_checked() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 4isize).into(), Color::White)))
            .build();
        board.set_possible_moves(Color::White);

        assert!(board.checked(Color::Black));
    }

    #[test]
    fn test_not_checked() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 5isize).into(), Color::White)))
            .build();
        board.set_possible_moves(Color::White);

        assert!(!board.checked(Color::Black));
    }
}
