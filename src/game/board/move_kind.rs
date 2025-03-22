use crate::game::pieces::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveKind {
    Attack(Option<PieceKind>), // A move that can take a piece
    CastleKingSide(PieceKind),
    CastleQueenSide(PieceKind),
    EnPassant(PieceKind),
    PawnSimpleMove, // A pawn move that can not take a piece (vertical pawn move)
    PawnDoubleMove, // A pawn move that can not take a piece (vertical pawn move)
    Promotion,
}
