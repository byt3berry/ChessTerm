use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;
use crate::pieces::move_struct::MoveKind;
use crate::pieces::{Move, Piece};

use super::color::Color;
use super::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct King {
    color: Color,
    position: Position,
    has_moved: bool,
}

impl King {
    pub const fn set_has_moved(&mut self) {
        self.has_moved = true;
    }

    fn queen_side_castling_final_position(&self) -> Position {
        (self.position.row(), 2usize).into()
    }

    fn king_side_castling_final_position(&self) -> Position {
        (self.position.row(), 6usize).into()
    }

    pub fn queen_side_castling_rook_position(&self) -> Position {
        (self.position.row(), 0usize).into()
    }

    pub fn king_side_castling_rook_position(&self) -> Position {
        (self.position.row(), 7usize).into()
    }
}

impl Piece for King {
    fn new(position: Position, color: Color) -> Self {
        Self {
            color,
            position,
            has_moved: false,
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn possible_moves(&self, board: &Board) -> HashSet<Move> {
        const OFFSETS: [(isize, isize); 8] = [
            (-1isize, -1isize),
            (-1isize, 0isize),
            (-1isize, 1isize),
            (0isize, -1isize),
            (0isize, 1isize),
            (1isize, -1isize),
            (1isize, 0isize),
            (1isize, 1isize),
        ];
        let mut output: HashSet<Move> = HashSet::new();
        let mut new_position: Position;
        let mut offset: (isize, isize);
        let mut castle_final_position: Position;
        let mut castle_rook_position: Position;

        for offset in OFFSETS {
            new_position = self.position + offset;

            if board.player(self.color.other()).is_attacking(new_position) {
                continue;
            }
            let Some(square) = board.square(new_position) else {
                continue;
            };
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.insert(Move::new(self.position, new_position, MoveKind::Attack));
                }
                continue;
            }

            output.insert(Move::new(self.position, new_position, MoveKind::Attack));
        }

        if self.has_moved {
            return output;
        }

        castle_final_position = self.queen_side_castling_final_position();
        castle_rook_position = self.queen_side_castling_rook_position();
        offset = (0isize, -1isize);
        new_position = self.position;

        if let Some(PieceKind::ROOK(rook)) = board.piece(castle_rook_position) {
            if rook.color() == self.color && !rook.has_moved() {
                loop {
                    new_position = new_position + offset;
                    if board.player(self.color.other()).is_attacking(new_position) {
                        break;
                    }

                    let Some(square) = board.square(new_position) else {
                        break;
                    };
                    if square.piece().is_some() {
                        break;
                    }

                    if new_position == castle_final_position {
                        output.insert(Move::new(self.position, new_position, MoveKind::CastleQueenSide));
                    }
                }
            }
        };

        castle_final_position = self.king_side_castling_final_position();
        castle_rook_position = self.king_side_castling_rook_position();
        offset = (0isize, 1isize);
        new_position = self.position;

        if let Some(PieceKind::ROOK(rook)) = board.piece(castle_rook_position) {
            if rook.color() == self.color && !rook.has_moved() {
                loop {
                    new_position = new_position + offset;
                    if board.player(self.color.other()).is_attacking(new_position) {
                        break;
                    }

                    let Some(square) = board.square(new_position) else {
                        break;
                    };
                    if square.piece().is_some() {
                        break;
                    }

                    if new_position == castle_final_position {
                        output.insert(Move::new(self.position, new_position, MoveKind::CastleKingSide));
                    }
                }
            }
        };

        output
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::board::Board;
    use crate::board::board_builder::BoardBuilder;
    use crate::pieces::bishop::Bishop;
    use crate::pieces::color::Color;
    use crate::pieces::move_struct::MoveKind;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::piece_kind::PieceKind;
    use crate::pieces::queen::Queen;
    use crate::pieces::rook::Rook;
    use crate::pieces::{Move, Piece};

    use super::King;

    #[test]
    fn test_simple_moves() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((3isize, 3isize).into(), Color::BLACK)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_moves_attacked_square_other_color() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((3isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((2isize, 0isize).into(), Color::WHITE)))
            .add(PieceKind::ROOK(Rook::new((4isize, 0isize).into(), Color::WHITE)))
            .add(PieceKind::ROOK(Rook::new((0isize, 2isize).into(), Color::WHITE)))
            .add(PieceKind::ROOK(Rook::new((0isize, 4isize).into(), Color::WHITE)))
            .build();
        board.set_attacking(Color::WHITE);
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece {position} should exist");
        let expected: HashSet<Move> = HashSet::new();

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_moves_attacked_square_same_color() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((3isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((2isize, 0isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((4isize, 0isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 4isize).into(), Color::BLACK)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_no_moves() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((3isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 4isize).into(), Color::BLACK)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece {position} should exist");
        let expected: HashSet<Move> = HashSet::new();

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_capture() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((3isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 2isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 4isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 2isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 4isize).into(), Color::WHITE)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_castle_king_side() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::QUEEN(Queen::new((0isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), Color::BLACK)))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 6isize).into(), MoveKind::CastleKingSide));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_castle_queen_side() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::BISHOP(Bishop::new((0isize, 5isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), Color::BLACK)))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 2isize).into(), MoveKind::CastleQueenSide));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_castle_king_moved() {
        let mut king: King = King::new((0isize, 4isize).into(), Color::BLACK);
        king.set_has_moved();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(king))
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), Color::BLACK)))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_castle_rook_moved() {
        let mut queen_side_rook: Rook = Rook::new((0isize, 0isize).into(), Color::BLACK);
        let mut king_side_rook: Rook = Rook::new((0isize, 7isize).into(), Color::BLACK);
        queen_side_rook.set_has_moved();
        king_side_rook.set_has_moved();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(queen_side_rook))
            .add(PieceKind::ROOK(king_side_rook))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_castle_attacked_same_color() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), Color::BLACK)))
            .build();
        board.set_attacking(Color::BLACK);
        board.set_attacking(Color::WHITE);
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 2isize).into(), MoveKind::CastleQueenSide));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 6isize).into(), MoveKind::CastleKingSide));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_castle_attacked_other_color() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new((0isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), Color::BLACK)))
            .add(PieceKind::ROOK(Rook::new((3isize, 2isize).into(), Color::WHITE)))
            .add(PieceKind::ROOK(Rook::new((3isize, 6isize).into(), Color::WHITE)))
            .build();
        board.set_attacking(Color::WHITE);
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }
}
