use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

use board::Board;
use board::color::Color;
use board::move_struct::Move;
use board::position::Position;
use board::square::Square;

pub(super) mod pieces;
pub(super) mod board;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Result {
    None,
    Checkmate,
    Stalemate,
    Draw,
}

pub struct ChessEngine {
    board: Board,
    current_player: Color,
    possible_moves: HashMap<Position, HashSet<Move>>,
    result: Result,
    moves: Vec<Move>,
    positions: Vec<u64>,
}

impl ChessEngine {
    pub fn new() -> Self {
        let starting_player: Color = Color::White;
        let board = Board::init();
        Self::from_board(board, starting_player)
    }
    
    pub fn from_board(mut board: Board, starting_player: Color) -> Self {
        board.set_possible_moves(starting_player);
        board.set_possible_moves(starting_player.other());

        let mut chess_engine = Self {
            board,
            current_player: starting_player,
            possible_moves: HashMap::new(),
            result: Result::None,
            moves: Vec::new(),
            positions: Vec::new(),
        };

        chess_engine.store_hash();
        chess_engine.set_possible_moves();
        chess_engine
    }

    pub fn is_end(&self) -> bool {
        !matches!(self.result, Result::None)
    }

    pub fn square(&self, position: Position) -> Option<&Square> {
        self.board.square(position)
    }

    pub fn try_move(&mut self, from: Position, to: Position) -> bool {
        let Some(possible_moves) = self.possible_moves.get(&from) else {
            return false;
        };
        let Some(try_move) = possible_moves
            .iter()
            .find(|m| m.from() == from && m.to() == to) else {
                return false;
            };

        self.board.make_move(try_move, self.current_player);
        self.moves.push(try_move.clone());
        let last_hash: u64 = self.store_hash();
        self.next_turn();

        if self.positions.iter().filter(|hash| last_hash.eq(hash)).count() == 3 {
            self.result = Result::Draw;
        }

        true
    }

    fn store_hash(&mut self) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        hasher.write_u64(0);
        self.hash(&mut hasher);
        let hash: u64 = hasher.finish();
        self.positions.push(hash);
        hash
    }

    fn next_turn(&mut self) {
        self.board.set_possible_moves(self.current_player);
        self.board.set_possible_moves(self.current_player.other());
        self.current_player = self.current_player.other();
        self.set_possible_moves();
    }

    pub fn checked_king(&self) -> Option<Position> {
        if self.board.checked(self.current_player) {
            self.board.king(self.current_player)
        } else {
            None
        }
    }

    pub fn set_possible_moves(&mut self) {
        self.possible_moves = self
            .board
            .pieces(self.current_player)
            .iter()
            .map(|piece| {
                let mut possible_moves: HashSet<Move> = piece.possible_moves(&self.board);
                self.filter_check_block(&mut possible_moves);
                (piece.position(), possible_moves)
            })
        .collect();

        if self.possible_moves.iter().all(|(_, possible_moves)| possible_moves.is_empty()) {
            if self.board.checked(self.current_player) {
                self.result = Result::Checkmate;
            } else {
                self.result = Result::Stalemate;
            }
        }
    }

    fn filter_check_block(&self, possible_moves: &mut HashSet<Move>) {
        possible_moves
            .retain(|simulated_move| {
                let mut simulated_board: Board = self
                    .board
                    .simulate_move(simulated_move, self.current_player);
                simulated_board.set_possible_moves(self.current_player.other());
                !simulated_board
                    .checked(self.current_player)
            });
    }

    pub fn possible_positions(&self, position: Option<Position>) -> Option<HashSet<Position>> {
        let possible_moves: &HashSet<Move> = self.possible_moves.get(&position?).as_ref()?;
        let position = position?;

        if self.board.piece(position, self.current_player).is_some() {
            return Some(possible_moves
                .iter()
                .map(|m| m.to())
                .collect());
        }

        None
    }
}

impl Hash for ChessEngine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::game::board::Board;
    use crate::game::board::board_builder::BoardBuilder;
    use crate::game::board::color::Color;
    use crate::game::board::position::Position;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::king::King;
    use crate::game::pieces::knight::Knight;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;
    use crate::game::pieces::queen::Queen;
    use crate::game::pieces::rook::Rook;
    use crate::game::Result;

    use super::ChessEngine;

    #[test]
    fn test_filter_check_block() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((7isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((6isize, 6isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Queen(Queen::new((4isize, 6isize).into(), Color::Black)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.try_move((4isize, 6isize).into(), (4isize, 7isize).into());
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((5isize, 6isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((6isize, 6isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_bishop_pinned_no_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Bishop(Bishop::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_bishop_pinned_can_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Bishop(Bishop::new((1isize, 1isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((2isize, 2isize).into());
        expected.insert((3isize, 3isize).into());
        expected.insert((4isize, 4isize).into());
        expected.insert((5isize, 5isize).into());
        expected.insert((6isize, 6isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((1isize, 1isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_king_moves_attacked_square_other_color() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((2isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((4isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((0isize, 2isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((0isize, 4isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_king_moves_attacked_square_same_color() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((2isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((4isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 4isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((2isize, 2isize).into());
        expected.insert((2isize, 3isize).into());
        expected.insert((2isize, 4isize).into());
        expected.insert((3isize, 2isize).into());
        expected.insert((3isize, 4isize).into());
        expected.insert((4isize, 2isize).into());
        expected.insert((4isize, 3isize).into());
        expected.insert((4isize, 4isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_knight_pinned_no_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_pawn_pinned_no_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_pawn_pinned_no_attack() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((4isize, 3isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_pawn_pinned_can_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((4isize, 3isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_pawn_pinned_can_en_passant() {
        let mut pawn: Pawn = Pawn::new((4isize, 3isize).into(), Color::White);
        pawn.set_en_passant_possible();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((7isize, 1isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(pawn))
            .add(PieceKind::Bishop(Bishop::new((6isize, 7isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((5isize, 3isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((4isize, 4isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_queen_pinned_can_move_straight() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((1isize, 3isize).into());
        expected.insert((2isize, 3isize).into());
        expected.insert((4isize, 3isize).into());
        expected.insert((5isize, 3isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_queen_pinned_can_move_diagonal() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((1isize, 1isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((2isize, 2isize).into());
        expected.insert((3isize, 3isize).into());
        expected.insert((4isize, 4isize).into());
        expected.insert((5isize, 5isize).into());
        expected.insert((6isize, 6isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((1isize, 1isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_rook_pinned_no_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Rook(Rook::new((1isize, 1isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((1isize, 1isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_rook_pinned_can_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Rook(Rook::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((1isize, 3isize).into());
        expected.insert((2isize, 3isize).into());
        expected.insert((4isize, 3isize).into());
        expected.insert((5isize, 3isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_result_checkmate() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((1isize, 0isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let expected: Result = Result::Checkmate;

        let result: Result = chess_game.result;

        assert_eq!(expected, result);
    }

    #[test]
    fn test_result_stalemate() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((2isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((7isize, 1isize).into(), Color::White)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let expected: Result = Result::Stalemate;

        let result: Result = chess_game.result;

        assert_eq!(expected, result);
    }

    #[test]
    fn test_result_draw_repetitions_in_a_row() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((7isize, 0isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.try_move((0isize, 0isize).into(), (0isize, 1isize).into());
        chess_game.try_move((7isize, 0isize).into(), (7isize, 1isize).into());
        chess_game.try_move((0isize, 1isize).into(), (0isize, 0isize).into());
        chess_game.try_move((7isize, 1isize).into(), (7isize, 0isize).into());
        chess_game.try_move((0isize, 0isize).into(), (0isize, 1isize).into());
        chess_game.try_move((7isize, 0isize).into(), (7isize, 1isize).into());
        chess_game.try_move((0isize, 1isize).into(), (0isize, 0isize).into());
        chess_game.try_move((7isize, 1isize).into(), (7isize, 0isize).into());
        let expected: Result = Result::Draw;

        let result: Result = chess_game.result;

        assert_eq!(expected, result);
    }

    #[test]
    fn test_result_draw_repetitions_separated() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((7isize, 0isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.try_move((0isize, 0isize).into(), (0isize, 1isize).into());
        chess_game.try_move((7isize, 0isize).into(), (7isize, 1isize).into());
        chess_game.try_move((0isize, 3isize).into(), (0isize, 4isize).into());
        chess_game.try_move((7isize, 1isize).into(), (7isize, 0isize).into());
        chess_game.try_move((0isize, 1isize).into(), (0isize, 0isize).into());
        chess_game.try_move((7isize, 0isize).into(), (7isize, 1isize).into());
        chess_game.try_move((0isize, 4isize).into(), (0isize, 3isize).into());
        chess_game.try_move((7isize, 1isize).into(), (7isize, 0isize).into());
        chess_game.try_move((0isize, 0isize).into(), (0isize, 1isize).into());
        chess_game.try_move((7isize, 0isize).into(), (7isize, 1isize).into());
        chess_game.try_move((0isize, 1isize).into(), (0isize, 0isize).into());
        chess_game.try_move((7isize, 1isize).into(), (7isize, 0isize).into());
        let expected: Result = Result::Draw;

        let result: Result = chess_game.result;

        assert_eq!(expected, result);
    }
}
