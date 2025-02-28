use board::Board;
use drawer::draw;

mod board;
mod drawer;
mod pieces;

fn main() {
    let board: Board = Board::init();
    draw(&board);
}
