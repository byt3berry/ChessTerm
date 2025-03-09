use std::collections::HashSet;

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
    pub(crate) const fn set_en_passant_possible(&mut self) {
        self.en_passant_possible = true;
    }

    pub(crate) const fn unset_en_passant_possible(&mut self) {
        self.en_passant_possible = false;
    }

    pub const fn set_has_moved(&mut self) {
        self.has_moved = true;
    }

    const fn direction(&self) -> (isize, isize) {
        match self.color {
            Color::White => (-1isize, 0isize),
            Color::Black => (1isize, 0isize),
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

    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn possible_moves(&self, board: &Board) -> HashSet<Move> {
        const OFFSETS: [(isize, isize); 2] = [
            (0isize, -1isize),
            (0isize, 1isize),
        ];
        let mut to: Position;
        let direction: (isize, isize) = self.direction();
        let mut output: HashSet<Move> = HashSet::new();

        // Simple moves
        if self.pinned_but_movable(board, direction.into()) {
            to = self.position + direction;
            if let Some(square) = board.square(to) {
                if square.piece().is_none() {
                    output.insert(Move::new(self.position, to, MoveKind::PawnSimpleMove, None));

                    if !self.has_moved {
                        to = to + direction;
                        if let Some(square) = board.square(to) {
                            if square.piece().is_none() {
                                output.insert(Move::new(self.position, to, MoveKind::PawnDoubleMove, None));
                            }
                        }
                    }
                }
            }
        }

        // Attack
        let mut new_offset: (isize, isize);
        for offset in OFFSETS {
            new_offset = add_offsets(direction, offset);

            if !self.pinned_but_movable(board, new_offset.into()) {
                continue;
            }

            to = self.position + new_offset;
            if let Some(piece) = board.piece(to) {
                if piece.color() != self.color {
                    output.insert(Move::new(self.position, to, MoveKind::Attack, None));
                }
            }
        }

        // En passant
        let mut pawn_position: Position;
        for offset in OFFSETS {
            new_offset = add_offsets(offset, direction);

            if !self.pinned_but_movable(board, new_offset.into()) {
                continue;
            }

            pawn_position = self.position + offset;
            to = self.position + new_offset;

            if let Some(PieceKind::Pawn(pawn)) = board.piece(pawn_position) {
                if pawn.en_passant_possible {
                    output.insert(Move::new(self.position, to, MoveKind::EnPassant(pawn_position), None));
                }
            }
        }

        output
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
    use crate::game::pieces::Piece;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::king::King;
    use crate::game::pieces::piece_kind::PieceKind;
    use crate::game::pieces::rook::Rook;

    use super::Pawn;

    #[test]
    fn test_simple_moves_not_moved() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((1isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((1isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((1isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::PawnSimpleMove, None));
        expected.insert(Move::new((1isize, 3isize).into(), (3isize, 3isize).into(), MoveKind::PawnDoubleMove, None));

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
        let mut pawn: Pawn = Pawn::new((2isize, 3isize).into(), Color::Black);
        pawn.set_has_moved();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(pawn))
            .build();
        let piece: &PieceKind = board
            .piece((2isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((2isize, 3isize).into(), (3isize, 3isize).into(), MoveKind::PawnSimpleMove, None));

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
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((1isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((2isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((1isize, 3isize).into())
            .expect("The piece should exist");
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
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((4isize, 2isize).into(), Color::White)))
            .add(PieceKind::Bishop(Bishop::new((4isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((4isize, 4isize).into(), Color::White)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack, None));

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
            .piece((4isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((4isize, 3isize).into(), (5isize, 2isize).into(), MoveKind::EnPassant((4isize, 2isize).into()), None));
        expected.insert(Move::new((4isize, 3isize).into(), (5isize, 4isize).into(), MoveKind::EnPassant((4isize, 4isize).into()), None));

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
    fn test_pinned_no_move() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        board.set_attacking(Color::White);
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
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
    fn test_pinned_no_attack() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((4isize, 3isize).into(), Color::White)))
            .build();
        board.set_attacking(Color::White);
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
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
    fn test_pinned_can_move() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        board.set_attacking(Color::White);
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::PawnSimpleMove, None));

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
    fn test_pinned_can_en_passant() {
        let mut pawn: Pawn = Pawn::new((4isize, 3isize).into(), Color::White);
        pawn.set_en_passant_possible();
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((7isize, 1isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(pawn))
            .add(PieceKind::Bishop(Bishop::new((6isize, 7isize).into(), Color::White)))
            .build();
        board.set_attacking(Color::White);
        let piece: &PieceKind = board
            .piece((4isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((4isize, 4isize).into(), (5isize, 3isize).into(), MoveKind::EnPassant((4isize, 3isize).into()), None));

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
