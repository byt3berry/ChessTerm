use game::board::Board;
use drawer::draw;

mod cursor;
mod drawer;
mod game;

fn main() {
    let board: Board = Board::init();
    draw(&board);
}
