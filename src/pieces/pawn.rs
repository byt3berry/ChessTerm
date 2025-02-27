use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;
use crate::pieces::{Color, Move, Piece};

use super::{MoveKind, PieceKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pawn {
    color: Color,
    position: Position,
    en_passant_possible: bool,
    has_moved: bool,
}

impl Pawn {
    pub const fn set_en_passant_possible(&mut self) {
        self.en_passant_possible = true;
    }

    pub const fn unset_en_passant_possible(&mut self) {
        self.en_passant_possible = false;
    }

    pub const fn set_has_moved(&mut self) {
        self.has_moved = true;
    }

    pub const fn direction(&self) -> (isize, isize) {
        match self.color {
            Color::WHITE => (-1isize, 0isize),
            Color::BLACK => (1isize, 0isize),
        }
    }
}


impl Piece for Pawn {
    fn new(position: Position, color: Color) -> Self {
        Self {
            color,
            position,
            en_passant_possible: false,
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
        const OFFSETS: [(isize, isize); 2] = [
            (0isize, -1isize),
            (0isize, 1isize),
        ];
        let mut output: HashSet<Move> = HashSet::new();
        let direction: (isize, isize) = self.direction();
        let mut new_position: Position;

        new_position = self.position + direction;
        if let Some(square) = board.square(new_position) {
            if square.piece().is_none() {
                output.insert(Move::new(self.position, new_position, MoveKind::Move));

                if !self.has_moved {
                    new_position = new_position + direction;
                    if let Some(square) = board.square(new_position) {
                        if square.piece().is_none() {
                            output.insert(Move::new(self.position, new_position, MoveKind::Move));
                        }
                    }
                }
            }
        }

        for offset in OFFSETS {
            new_position = self.position + direction + offset;
            if let Some(piece) = board.piece(new_position) {
                if piece.color() != self.color {
                    output.insert(Move::new(self.position, new_position, MoveKind::Capture));
                }
            }
        }

        for offset in OFFSETS {
            new_position = self.position + offset;
            if let Some(PieceKind::PAWN(pawn)) = board.piece(new_position) {
                if pawn.en_passant_possible {
                    output.insert(Move::new(self.position, new_position + direction, MoveKind::EnPassant));
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::board::board_builder::BoardBuilder;
    use crate::pieces::bishop::Bishop;
    use crate::pieces::{Color, Move, MoveKind, Piece, PieceKind};

    use super::Pawn;

    #[test]
    fn test_simple_moves_not_moved() {
        let position = (1usize, 3usize).into();
        let color = Color::BLACK;
        let board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new(position, color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (2usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 3usize).into(), MoveKind::Move));

        let possible_moves = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_simple_moves_moved() {
        let position = (2usize, 3usize).into();
        let color = Color::BLACK;
        let mut pawn: Pawn = Pawn::new(position, color);
        pawn.set_has_moved();
        let board = BoardBuilder::new()
            .add(PieceKind::PAWN(pawn))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (3usize, 3usize).into(), MoveKind::Move));

        let possible_moves = piece.possible_moves(&board);

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
        let position = (1usize, 3usize).into();
        let color = Color::BLACK;
        let board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new(position, color)))
            .add(PieceKind::BISHOP(Bishop::new((2isize, 3isize).into(), color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let expected: HashSet<Move> = HashSet::new();

        let possible_moves = piece.possible_moves(&board);

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
        let position = (3usize, 3usize).into();
        let color = Color::BLACK;
        let board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new(position, color)))
            .add(PieceKind::BISHOP(Bishop::new((4isize, 2isize).into(), color.other())))
            .add(PieceKind::BISHOP(Bishop::new((4isize, 3isize).into(), color)))
            .add(PieceKind::BISHOP(Bishop::new((4isize, 4isize).into(), color.other())))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (4isize, 2isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (4isize, 4isize).into(), MoveKind::Capture));

        let possible_moves = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_en_passant() {
        let position = (4usize, 3usize).into();
        let color = Color::BLACK;
        let mut pawn_left: Pawn = Pawn::new((4isize, 2isize).into(), color.other());
        let mut pawn_right: Pawn = Pawn::new((4isize, 4isize).into(), color.other());
        pawn_left.set_en_passant_possible();
        pawn_right.set_en_passant_possible();
        let board = BoardBuilder::new()
            .add(PieceKind::PAWN(Pawn::new(position, color)))
            .add(PieceKind::BISHOP(Bishop::new((5isize, 3isize).into(), color)))
            .add(PieceKind::PAWN(pawn_left))
            .add(PieceKind::PAWN(pawn_right))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (5isize, 2isize).into(), MoveKind::EnPassant));
        expected.insert(Move::new(position, (5isize, 4isize).into(), MoveKind::EnPassant));

        let possible_moves = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }
}
