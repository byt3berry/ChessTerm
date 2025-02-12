use board::Board;
use drawer::drawable::{draw_piece, BG_BLACK, BG_WHITE};
use pieces::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook, Color, Piece, PieceKind};

mod board;
mod drawer;
mod pieces;
mod player;

fn main() {
    let board: Board = Board::new();

    let piece: PieceKind = PieceKind::ROOK(Rook::new(Color::WHITE));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::QUEEN(Queen::new(Color::WHITE));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::PAWN(Pawn::new(Color::WHITE));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::KNIGHT(Knight::new(Color::WHITE));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::KING(King::new(Color::WHITE));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::BISHOP(Bishop::new(Color::WHITE));
    draw_piece(&piece, BG_WHITE);

    let piece: PieceKind = PieceKind::ROOK(Rook::new(Color::BLACK));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::QUEEN(Queen::new(Color::BLACK));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::PAWN(Pawn::new(Color::BLACK));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::KNIGHT(Knight::new(Color::BLACK));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::KING(King::new(Color::BLACK));
    draw_piece(&piece, BG_WHITE);
    let piece: PieceKind = PieceKind::BISHOP(Bishop::new(Color::BLACK));
    draw_piece(&piece, BG_WHITE);

    let piece: PieceKind = PieceKind::ROOK(Rook::new(Color::WHITE));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::QUEEN(Queen::new(Color::WHITE));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::PAWN(Pawn::new(Color::WHITE));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::KNIGHT(Knight::new(Color::WHITE));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::KING(King::new(Color::WHITE));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::BISHOP(Bishop::new(Color::WHITE));
    draw_piece(&piece, BG_BLACK);

    let piece: PieceKind = PieceKind::ROOK(Rook::new(Color::BLACK));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::QUEEN(Queen::new(Color::BLACK));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::PAWN(Pawn::new(Color::BLACK));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::KNIGHT(Knight::new(Color::BLACK));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::KING(King::new(Color::BLACK));
    draw_piece(&piece, BG_BLACK);
    let piece: PieceKind = PieceKind::BISHOP(Bishop::new(Color::BLACK));
    draw_piece(&piece, BG_BLACK);
}
