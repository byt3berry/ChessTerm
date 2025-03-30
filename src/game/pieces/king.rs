use std::collections::HashSet;
use std::hash::{Hash, Hasher};

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
    pub const fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn with_has_moved(mut self) -> Self {
        self.has_moved = true;
        self
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

    pub const fn set_has_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn original_position(&self) -> Position {
        match self.color {
            Color::White => (7isize, 4isize).into(),
            Color::Black => (0isize, 4isize).into(),
            Color::Any => panic!("A king of color \"Any\" has no original position"),
        }
    }
}

impl Hash for King {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.position.hash(state);
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

    fn points(&self) -> i16 {
        100i16
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
        let mut rook_piece: Option<&PieceKind>;
        let mut castle_final_position: Position;
        let mut castle_rook_position: Position;
        let mut offset: (isize, isize);
        let mut output: HashSet<Move> = HashSet::new();

        for offset in OFFSETS {
            to = self.position + offset;

            let Some(square) = board.square(to) else {
                continue;
            };
            if let Some(piece) = square.piece(Color::Any) {
                if piece.color() != self.color() {
                    output.insert(Move::new(self.position, to, MoveKind::Attack(Some(*piece))));
                }
                continue;
            }

            output.insert(Move::new(self.position, to, MoveKind::Attack(None)));
        }

        if self.has_moved || self.position != self.original_position() {
            return output;
        }

        castle_final_position = self.queen_side_castling_final_position();
        castle_rook_position = self.queen_side_castling_rook_position();
        rook_piece = board.piece(castle_rook_position, self.color);
        offset = (0isize, -1isize);
        to = self.position;

        if let Some(rook) = rook_piece {
            if matches!(rook, PieceKind::Rook(rook) if !rook.has_moved()) {
                loop {
                    to = to + offset;
                    if board.player(self.color.other()).is_attacking(to) {
                        break;
                    }

                    let Some(square) = board.square(to) else {
                        break;
                    };
                    if square.piece(Color::Any).is_some() {
                        break;
                    }

                    if to == castle_final_position {
                        output.insert(Move::new(self.position, to, MoveKind::CastleQueenSide(*rook)));
                    }
                }
            }
        };

        castle_final_position = self.king_side_castling_final_position();
        castle_rook_position = self.king_side_castling_rook_position();
        rook_piece = board.piece(castle_rook_position, self.color);
        offset = (0isize, 1isize);
        to = self.position;

        if let Some(rook) = rook_piece {
            if matches!(rook, PieceKind::Rook(rook) if !rook.has_moved()) {
                loop {
                    to = to + offset;
                    if board.player(self.color.other()).is_attacking(to) {
                        break;
                    }

                    let Some(square) = board.square(to) else {
                        break;
                    };
                    if square.piece(Color::Any).is_some() {
                        break;
                    }

                    if to == castle_final_position {
                        output.insert(Move::new(self.position, to, MoveKind::CastleKingSide(*rook)));
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
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;
    use crate::game::pieces::queen::Queen;
    use crate::game::pieces::rook::Rook;
    use crate::game::pieces::Piece;

    use super::King;

    #[test]
    fn test_simple_moves() {
        let board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_no_moves() {
        let board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((2isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
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
        let pawn1: PieceKind = PieceKind::Pawn(Pawn::new((2isize, 2isize).into(), Color::White));
        let pawn2: PieceKind = PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::White));
        let pawn3: PieceKind = PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::White));
        let pawn4: PieceKind = PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::White));
        let pawn5: PieceKind = PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::White));
        let pawn6: PieceKind = PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::White));
        let pawn7: PieceKind = PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::White));
        let pawn8: PieceKind = PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White));
        let board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .with(pawn1)
            .with(pawn2)
            .with(pawn3)
            .with(pawn4)
            .with(pawn5)
            .with(pawn6)
            .with(pawn7)
            .with(pawn8)
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack(Some(pawn1))));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack(Some(pawn2))));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack(Some(pawn3))));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack(Some(pawn4))));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack(Some(pawn5))));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack(Some(pawn6))));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack(Some(pawn7))));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack(Some(pawn8))));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_castle_king_side() {
        let rook: PieceKind = PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black));
        let board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Queen(Queen::new((0isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .with(rook)
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 6isize).into(), MoveKind::CastleKingSide(rook)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_castle_queen_side() {
        let rook: PieceKind = PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black));
        let board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Bishop(Bishop::new((0isize, 5isize).into(), Color::Black)))
            .with(rook)
            .with(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 2isize).into(), MoveKind::CastleQueenSide(rook)));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_castle_king_moved() {
        let board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black).with_has_moved()))
            .with(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_castle_rook_moved() {
        let board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black).with_has_moved()))
            .with(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black).with_has_moved()))
            .build();
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_castle_attacked_same_color() {
        let rook1: PieceKind = PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black));
        let rook2: PieceKind = PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black));
        let mut board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .with(rook1)
            .with(rook2)
            .build();
        board.set_possible_moves(Color::Black);
        board.set_possible_moves(Color::White);
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 2isize).into(), MoveKind::CastleQueenSide(rook1)));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 6isize).into(), MoveKind::CastleKingSide(rook2)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_castle_attacked_other_color() {
        let mut board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((3isize, 2isize).into(), Color::White)))
            .with(PieceKind::Rook(Rook::new((3isize, 6isize).into(), Color::White)))
            .build();
        board.set_possible_moves(Color::White);
        let piece: &PieceKind = board
            .piece((0isize, 4isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (0isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((0isize, 4isize).into(), (1isize, 5isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }
}
