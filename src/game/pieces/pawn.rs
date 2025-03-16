use std::collections::HashSet;
use std::hash::{self, Hasher};

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::move_kind::MoveKind;
use crate::game::board::move_struct::Move;
use crate::game::board::position::Position;

use super::Piece;
use super::piece_kind::PieceKind;

fn add_offsets(offset1: (isize, isize), offset2: (isize, isize)) -> (isize, isize) {
    (offset1.0+ offset2.0, offset1.1 + offset2.1)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Pawn {
    color: Color,
    position: Position,
    en_passant_possible: bool,
    has_moved: bool,
}

impl Pawn {
    pub fn new(position: Position, color: Color) -> Self {
        Self {
            color,
            position,
            en_passant_possible: false,
            has_moved: false,
        }
    }

    const fn direction(&self) -> (isize, isize) {
        match self.color {
            Color::White => (-1isize, 0isize),
            Color::Black => (1isize, 0isize),
            Color::Any => panic!("A pawn of color \"Any\" has no direction"),
        }
    }

    pub const fn set_en_passant_possible(&mut self) {
        self.en_passant_possible = true;
    }

    pub const fn unset_en_passant_possible(&mut self) {
        self.en_passant_possible = false;
    }

    pub const fn set_has_moved(&mut self) {
        self.has_moved = true;
    }
}

impl hash::Hash for Pawn {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.position.hash(state);
    }
}

impl Piece for Pawn {
    fn possible_moves(&self, board: &Board) -> HashSet<Move> {
        const OFFSETS: [(isize, isize); 2] = [
            (0isize, -1isize),
            (0isize, 1isize),
        ];
        let mut to: Position;
        let direction: (isize, isize) = self.direction();
        let mut output: HashSet<Move> = HashSet::new();

        // Simple moves
        to = self.position + direction;
        if board.piece(to, Color::Any).is_none() {
            output.insert(Move::new(self.position, to, MoveKind::PawnSimpleMove));

            if !self.has_moved {
                to = to + direction;
                if board.piece(to, Color::Any).is_none() {
                    output.insert(Move::new(self.position, to, MoveKind::PawnDoubleMove));
                }
            }
        }

        // Attack
        let mut new_offset: (isize, isize);
        for offset in OFFSETS {
            new_offset = add_offsets(direction, offset);

            to = self.position + new_offset;
            if board.piece(to, self.color.other()).is_some() {
                output.insert(Move::new(self.position, to, MoveKind::Attack));
            }
        }

        // En passant
        let mut pawn_position: Position;
        for offset in OFFSETS {
            new_offset = add_offsets(offset, direction);

            pawn_position = self.position + offset;
            to = self.position + new_offset;

            if board.piece(pawn_position, self.color.other())
                .is_some_and(|piece| matches!(piece, PieceKind::Pawn(pawn) if pawn.en_passant_possible)) 
                {
                    output.insert(Move::new(self.position, to, MoveKind::EnPassant(pawn_position)));
                }
        }

        output
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
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::game::board::Board;
    use crate::game::board::board_builder::BoardBuilder;
    use crate::game::board::color::Color;
    use crate::game::board::move_kind::MoveKind;
    use crate::game::board::move_struct::Move;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::piece_kind::PieceKind;

    use super::Pawn;

    #[test]
    fn test_simple_moves_not_moved() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((1isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((1isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((1isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::PawnSimpleMove));
        expected.insert(Move::new((1isize, 3isize).into(), (3isize, 3isize).into(), MoveKind::PawnDoubleMove));

        let possible_moves = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_simple_moves_moved() {
        let mut pawn: Pawn = Pawn::new((2isize, 3isize).into(), Color::Black);
        pawn.set_has_moved();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(pawn))
            .build();
        let piece: &PieceKind = board
            .piece((2isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((2isize, 3isize).into(), (3isize, 3isize).into(), MoveKind::PawnSimpleMove));

        let possible_moves = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_no_moves() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((1isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((2isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((1isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let expected: HashSet<Move> = HashSet::new();

        let possible_moves = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_capture() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((4isize, 2isize).into(), Color::White)))
            .add(PieceKind::Bishop(Bishop::new((4isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((4isize, 4isize).into(), Color::White)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack));

        let possible_moves = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_en_passant() {
        let mut pawn_left: Pawn = Pawn::new((4isize, 2isize).into(), Color::White);
        let mut pawn_right: Pawn = Pawn::new((4isize, 4isize).into(), Color::White);
        pawn_left.set_en_passant_possible();
        pawn_right.set_en_passant_possible();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((5isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(pawn_left))
            .add(PieceKind::Pawn(pawn_right))
            .build();
        let piece: &PieceKind = board
            .piece((4isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((4isize, 3isize).into(), (5isize, 2isize).into(), MoveKind::EnPassant((4isize, 2isize).into())));
        expected.insert(Move::new((4isize, 3isize).into(), (5isize, 4isize).into(), MoveKind::EnPassant((4isize, 4isize).into())));

        let possible_moves = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }
}
