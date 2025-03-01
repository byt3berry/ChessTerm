use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;
use crate::pieces::move_struct::MoveKind;
use crate::pieces::{Move, Piece};

use super::color::Color;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rook {
    color: Color,
    position: Position,
    has_moved: bool,
}

impl Rook {
    pub const fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub const fn set_has_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn queen_side_castling_final_position(&self) -> Position {
        (self.position.row(), 3usize).into()
    }

    pub fn king_side_castling_final_position(&self) -> Position {
        (self.position.row(), 5usize).into()
    }
}

impl Piece for Rook {
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
        const OFFSETS: [(isize, isize); 4] = [
            (-1isize, 0isize),
            (0isize, -1isize),
            (0isize, 1isize),
            (1isize, 0isize),
        ];
        let mut output: HashSet<Move> = HashSet::new();
        let mut new_position: Position;

        for offset in OFFSETS {
            new_position = self.position;

            loop {
                new_position = new_position + offset;

                let Some(square) = board.square(new_position) else {
                    break;
                };
                if let Some(piece) = square.piece() {
                    if piece.color() != self.color() {
                        output.insert(Move::new(self.position, new_position, MoveKind::Attack));
                    }
                    break;
                }
                output.insert(Move::new(self.position, new_position, MoveKind::Attack));
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::board::Board;
    use crate::board::board_builder::BoardBuilder;
    use crate::pieces::color::Color;
    use crate::pieces::move_struct::MoveKind;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::piece_kind::PieceKind;
    use crate::pieces::{Move, Piece};

    use super::Rook;

    #[test]
    fn test_simple_moves() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::ROOK(Rook::new((3isize, 3isize).into(), Color::BLACK)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (0isize, 3isize).into(), MoveKind::Attack));
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
            .add(PieceKind::ROOK(Rook::new((3isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), Color::BLACK)))
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
            .add(PieceKind::ROOK(Rook::new((3isize, 3isize).into(), Color::BLACK)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), Color::WHITE)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), Color::WHITE)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack));

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
