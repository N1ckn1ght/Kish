mod util;
mod maps;
mod board;
mod engine;
mod interface;

fn main() {
    println!("Hello, world!");
    let _ = &*maps::MAGIC_MAPS;
    println!("Force init completed.");

    let mut bb: u64 = 0b111111111111110011111111111111111111;
    
}