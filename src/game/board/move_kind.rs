use super::position::Position;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveKind {
    Attack, // A move that can take a piece
    CastleKingSide,
    CastleQueenSide,
    EnPassant(Position),
    PawnSimpleMove, // A pawn move that can not take a piece (vertical pawn move)
    PawnDoubleMove, // A pawn move that can not take a piece (vertical pawn move)
    Promotion,
}
