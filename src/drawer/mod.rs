use crate::board::position::Position;
use crate::board::{Board, COLUMNS, ROWS};
use crate::board::square::Square;
use crate::pieces::Color;

mod pieces;
mod square;

const PIECE_BLACK: u8 = 235;
const PIECE_WHITE: u8 = 240;
const SQUARE_BLACK: u8 = 0;
const SQUARE_WHITE: u8 = 255;
const SQUARE_SIZE: usize = 22;

pub trait Drawable {
    fn get_drawing(&self) -> [u8; SQUARE_SIZE*SQUARE_SIZE];
}

pub fn draw(board: &Board) -> () {
    let mut position: Position;
    clean();

    for i in 0..ROWS {
        for j in 0..COLUMNS {
            position = (i, j).into();

            let Some(square) = board.get(&position) else {
                panic!("The square ({i}, {j}) should exist");
            };

            draw_square(square, position);
        }
    }
}

pub fn square_color(row: usize, column:usize) -> Color {
    assert!(row < ROWS);
    assert!(column < COLUMNS);

    if row % 2 == 0 && column % 2 == 0 || row % 2 == 1 && column % 2 == 1 {
        Color::WHITE
    } else {
        Color::BLACK
    }
}

fn draw_square(square: &Square, position: Position) -> () {
    let (row, column) = position.into();
    let drawing = square.get_drawing();
    let bg_color: u8 = match square_color(row, column) {
        Color::WHITE => SQUARE_WHITE,
        Color::BLACK => SQUARE_BLACK,
    };
    let piece_color: u8 = match square.piece() {
        Some(piece) => {
            match piece.color() {
                Color::WHITE => PIECE_WHITE,
                Color::BLACK => PIECE_BLACK,
            }
        },
        None => bg_color,
    };
    
    for i in 0..SQUARE_SIZE {
        goto(row*SQUARE_SIZE+i, column*SQUARE_SIZE*2);
        background(bg_color);

        for j in 0..SQUARE_SIZE {
            if drawing[i*SQUARE_SIZE+j] == 1 {
                background(piece_color);
                print!("  ");
                background(bg_color);
            } else {
                print!("  ");
            }
        }
        reset();
    }
}

fn goto(row: usize, column: usize) -> () {
    print!("\u{001b}[{row};{column}H");
}

fn background(color: u8) -> () {
    print!("\u{001b}[48;5;{color}m");
}

fn clean() {
    print!("\u{001b}[2J");
}

fn reset() -> () {
    print!("\u{001b}[0m")
}
