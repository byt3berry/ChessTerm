use std::collections::HashSet;

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::move_kind::MoveKind;
use crate::game::board::move_struct::Move;
use crate::game::board::position::Position;

use super::Piece;
use super::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct King {
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

    pub(crate) fn queen_side_castling_rook_position(&self) -> Position {
        (self.position.row(), 0usize).into()
    }

    pub(crate) fn king_side_castling_rook_position(&self) -> Position {
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
        let mut to: Position;
        let mut castle_final_position: Position;
        let mut castle_rook_position: Position;
        let mut offset: (isize, isize);
        let mut output: HashSet<Move> = HashSet::new();

        for offset in OFFSETS {
            to = self.position + offset;

            if board.player(self.color.other()).is_attacking(to) {
                continue;
            }
            let Some(square) = board.square(to) else {
                continue;
            };
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.insert(Move::new(self.position, to, MoveKind::Attack, None));
                }
                continue;
            }

            output.insert(Move::new(self.position, to, MoveKind::Attack, None));
        }

        if self.has_moved {
            return output;
        }

        castle_final_position = self.queen_side_castling_final_position();
        castle_rook_position = self.queen_side_castling_rook_position();
        offset = (0isize, -1isize);
        to = self.position;

        if let Some(PieceKind::Rook(rook)) = board.piece(castle_rook_position) {
            if rook.color() == self.color && !rook.has_moved() {
                loop {
                    to = to + offset;
                    if board.player(self.color.other()).is_attacking(to) {
                        break;
                    }

                    let Some(square) = board.square(to) else {
                        break;
                    };
                    if square.piece().is_some() {
                        break;
                    }

                    if to == castle_final_position {
                        output.insert(Move::new(self.position, to, MoveKind::CastleQueenSide, None));
                    }
                }
            }
        };

        castle_final_position = self.king_side_castling_final_position();
        castle_rook_position = self.king_side_castling_rook_position();
        offset = (0isize, 1isize);
        to = self.position;

        if let Some(PieceKind::Rook(rook)) = board.piece(castle_rook_position) {
            if rook.color() == self.color && !rook.has_moved() {
                loop {
                    to = to + offset;
                    if board.player(self.color.other()).is_attacking(to) {
                        break;
                    }

                    let Some(square) = board.square(to) else {
                        break;
                    };
                    if square.piece().is_some() {
                        break;
                    }

                    if to == castle_final_position {
                        output.insert(Move::new(self.position, to, MoveKind::CastleKingSide, None));
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

    use crate::game::board::Board;
    use crate::game::board::board_builder::BoardBuilder;
    use crate::game::board::color::Color;
    use crate::game::board::move_kind::MoveKind;
    use crate::game::board::move_struct::Move;
    use crate::game::pieces::Piece;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;
    use crate::game::pieces::queen::Queen;
    use crate::game::pieces::rook::Rook;

    use super::King;

    #[test]
    fn test_simple_moves() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack, None));

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
            .add(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((2isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((4isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((0isize, 2isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((0isize, 4isize).into(), Color::White)))
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
    fn test_moves_attacked_square_same_color() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((2isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((4isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 4isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack, None));

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
            .add(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
            .build();
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
    fn test_capture() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack, None));

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
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Queen(Queen::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 6isize).into(), MoveKind::CastleKingSide, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack, None));

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
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((0isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 2isize).into(), MoveKind::CastleQueenSide, None));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack, None));

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
        let mut king: King = King::new((0isize, 4isize).into(), Color::Black);
        king.set_has_moved();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(king))
            .add(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack, None));

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
        let mut queen_side_rook: Rook = Rook::new((0isize, 0isize).into(), Color::Black);
        let mut king_side_rook: Rook = Rook::new((0isize, 7isize).into(), Color::Black);
        queen_side_rook.set_has_moved();
        king_side_rook.set_has_moved();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Rook(queen_side_rook))
            .add(PieceKind::Rook(king_side_rook))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack, None));

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
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .build();
        board.set_attacking(Color::Black);
        board.set_attacking(Color::White);
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 2isize).into(), MoveKind::CastleQueenSide, None));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 6isize).into(), MoveKind::CastleKingSide, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack, None));

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
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((3isize, 2isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((3isize, 6isize).into(), Color::White)))
            .build();
        board.set_attacking(Color::White);
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack, None));

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
