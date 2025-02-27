use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;
use crate::pieces::MoveKind;
use crate::pieces::{Color, Move, Piece};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Queen {
    color: Color,
    position: Position,
}

impl Piece for Queen {
    fn new(position: Position, color: Color) -> Self {
        Self {
            color,
            position,
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> &Position {
        &self.position
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

        for offset in OFFSETS {
            new_position = self.position;

            loop {
                new_position = new_position + offset;

                let Some(square) = board.square(new_position) else {
                    break;
                };
                if let Some(piece) = square.piece() {
                    if piece.color() != self.color() {
                        output.insert(Move::new(self.position, new_position, MoveKind::Capture));
                    }
                    break;
                }
                output.insert(Move::new(self.position, new_position, MoveKind::Move));
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

    use super::Queen;

    #[test]
    fn test_simple_moves() {
        let position: Position = (3usize, 3usize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::QUEEN(Queen::new(position, color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (0usize, 0usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (0usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (0usize, 6usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 1usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 5usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (2usize, 2usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (2usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (2usize, 4usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 0usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 1usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 2usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 4usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 5usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 6usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 7usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (4usize, 2usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (4usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (4usize, 4usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (5usize, 1usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (5usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (5usize, 5usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (6usize, 0usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (6usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (6usize, 6usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (7usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (7usize, 7usize).into(), MoveKind::Move));

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
        let position: Position = (3usize, 3usize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::QUEEN(Queen::new(position, color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 2isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 4isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 2isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 4isize).into(), color)))
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
        let position: Position = (3usize, 3usize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::QUEEN(Queen::new(position, color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 2isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((2isize, 4isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((4isize, 2isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((4isize, 4isize).into(), color.other())))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (2isize, 2isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (2isize, 3isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (2isize, 4isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (3isize, 2isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (3isize, 4isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (4isize, 2isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (4isize, 3isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (4isize, 4isize).into(), MoveKind::Capture));

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
