use board::Board;

use std::time::Instant;
use std::hint::black_box;

mod util;
mod magic;
mod board;

fn main() {
    println!("Hello, world!");
    let _ = &*magic::MAGIC_MAPS;
    println!("Force init completed.");

    let mut board = Board::default();
    board.bb = 0b111111111111110011111111111111111111;

    
}
