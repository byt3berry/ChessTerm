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
    for i in 0..ROWS {
        for j in 0..COLUMNS {
            let Some(square) = board.get(i, j) else {
                panic!("The square ({i}, {j}) should exist");
            };

            draw_square(square, i*SQUARE_SIZE, j*SQUARE_SIZE*2);
        }
    }
}

fn draw_square(square: &Square, x: usize, y: usize) -> () {
    let drawing = square.get_drawing();
    let bg_color: u8 = match square.color() {
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
        goto(x+i, y);
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

fn reset() -> () {
    print!("\u{001b}[0m")
}
