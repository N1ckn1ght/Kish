use board::Board;

mod util;
mod magic;
mod board;

fn main() {
    println!("Hello, world!");

    let board = Board::default();
    println!("{}", board::MAGICS[0]);
}
