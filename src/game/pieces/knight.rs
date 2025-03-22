use std::collections::HashSet;

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::move_kind::MoveKind;
use crate::game::board::move_struct::Move;
use crate::game::board::position::Position;

use super::Piece;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Knight {
    color: Color,
    position: Position,
}

impl Knight {
    pub fn new(position: Position, color: Color) -> Self {
        Self {
            color,
            position,
        }
    }
}

impl Piece for Knight {
    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Position {
        self.position
    }

    fn points(&self) -> i8 {
        3i8
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn possible_moves(&self, board: &Board) -> HashSet<Move> {
        const OFFSETS: [(isize, isize); 8] = [
            (-1isize, -2isize),
            (-1isize, 2isize),
            (-2isize, -1isize),
            (-2isize, 1isize),
            (1isize, -2isize),
            (1isize, 2isize),
            (2isize, -1isize),
            (2isize, 1isize),
        ];
        let mut to: Position;
        let mut output: HashSet<Move> = HashSet::new();

        for offset in OFFSETS {
            to = self.position + offset;

            let Some(square) = board.square(to) else {
                continue;
            };
            if let Some(piece) = square.piece(Color::Any) {
                if piece.color() != self.color() {
                    output.insert(Move::new(self.position, to, MoveKind::Attack(Some(piece.clone()))));
                }
                continue;
            }
            output.insert(Move::new(self.position, to, MoveKind::Attack(None)));
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
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;

    use super::Knight;

    #[test]
    fn test_simple_moves() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 2isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 1isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 1isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 2isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 4isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_no_moves() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 1isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 1isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 4isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let expected: HashSet<Move> = HashSet::new();

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_capture() {
        let pawn1: PieceKind = PieceKind::Pawn(Pawn::new((1isize, 2isize).into(), Color::White));
        let pawn2: PieceKind = PieceKind::Pawn(Pawn::new((1isize, 4isize).into(), Color::White));
        let pawn3: PieceKind = PieceKind::Pawn(Pawn::new((2isize, 1isize).into(), Color::White));
        let pawn4: PieceKind = PieceKind::Pawn(Pawn::new((2isize, 5isize).into(), Color::White));
        let pawn5: PieceKind = PieceKind::Pawn(Pawn::new((4isize, 1isize).into(), Color::White));
        let pawn6: PieceKind = PieceKind::Pawn(Pawn::new((4isize, 5isize).into(), Color::White));
        let pawn7: PieceKind = PieceKind::Pawn(Pawn::new((5isize, 2isize).into(), Color::White));
        let pawn8: PieceKind = PieceKind::Pawn(Pawn::new((5isize, 4isize).into(), Color::White));
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::Black)))
            .add(pawn1)
            .add(pawn2)
            .add(pawn3)
            .add(pawn4)
            .add(pawn5)
            .add(pawn6)
            .add(pawn7)
            .add(pawn8)
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 2isize).into(), MoveKind::Attack(Some(pawn1))));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 4isize).into(), MoveKind::Attack(Some(pawn2))));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 1isize).into(), MoveKind::Attack(Some(pawn3))));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 5isize).into(), MoveKind::Attack(Some(pawn4))));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 1isize).into(), MoveKind::Attack(Some(pawn5))));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 5isize).into(), MoveKind::Attack(Some(pawn6))));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 2isize).into(), MoveKind::Attack(Some(pawn7))));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 4isize).into(), MoveKind::Attack(Some(pawn8))));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }
}
