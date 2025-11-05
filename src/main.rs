use board::magic::*;

mod board;
mod util;

fn main() {
    println!("Hello, world!");

    let blockers = init_blocker_boards();
    let combinations = init_combinations(&blockers);
    let attacks = init_attacks(&combinations);
    let solutions = init_solutions();

    let mut seed = 1;

    let magic  = search_for_magic(0, 8, combinations[0], attacks[0], &mut seed, 1048576);

    println!("final seed {}", seed);
    println!("magic {}", magic)
}
