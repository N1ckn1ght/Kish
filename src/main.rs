use board::magic::*;

mod board;
mod util;

fn main() {
    println!("Hello, world!");

    let blockers = init_blocker_boards();
    for i in 0..36 {
        let combs = init_combinations(blockers[i]);
        init_attacks(i, &combs);
    }
}
