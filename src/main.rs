use board::Board;
use drawer::draw;

mod board;
mod drawer;
mod pieces;
mod player;

fn main() {
    let board: Board = Board::init();
    draw(&board);
}
