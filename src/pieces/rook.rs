use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;
use crate::pieces::MoveKind;
use crate::pieces::{Color, Move, Piece};

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

    fn position(&self) -> &Position {
        &self.position
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
    use std::collections::HashSet;

    use crate::board::board_builder::BoardBuilder;
    use crate::board::position::Position;
    use crate::board::Board;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::{Color, Move, MoveKind, Piece, PieceKind};

    use super::Rook;

    #[test]
    fn test_simple_moves() {
        let position: Position = (3isize, 3isize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::ROOK(Rook::new(position, color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (0isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (1isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (2isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (3isize, 0isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (3isize, 1isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (3isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (3isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (3isize, 5isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (3isize, 6isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (3isize, 7isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (4isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (5isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (6isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (7isize, 3isize).into(), MoveKind::Attack));

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
        let position: Position = (3isize, 3isize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::ROOK(Rook::new(position, color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
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
        let position: Position = (3isize, 3isize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::ROOK(Rook::new(position, color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), color.other())))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (2isize, 3isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (3isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (3isize, 4isize).into(), MoveKind::Attack));
        expected.insert(Move::new(position, (4isize, 3isize).into(), MoveKind::Attack));

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
