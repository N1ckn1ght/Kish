use board::magic::*;
use util::{bb_to_str, print_boards};

mod board;
mod util;

fn main() {
    println!("Hello, world!");

    let blockers = init_blocker_boards();

    let i = 0;
    let j = 100;
    println!("{} {}", i, j);
    let combs = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));

    let i = 7;
    let j = 85;
    println!("{} {}", i, j);
    let combs = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));

    let i = 35;
    let j = 90;
    println!("{} {}", i, j);
    let combs = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));

    let i = 35;
    let j = 165;
    println!("{} {}", i, j);
    let combs: [u64; 256] = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));

    let i = 15;
    let j = 157;
    println!("{} {}", i, j);
    let combs = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));

    let i = 15;
    let j = 185;
    println!("{} {}", i, j);
    let combs = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));

    let i = 22;
    let j = 106;
    println!("{} {}", i, j);
    let combs = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));

    let i = 22;
    let j = 157;
    println!("{} {}", i, j);
    let combs = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));

    let i = 22;
    let j = 185;
    println!("{} {}", i, j);
    let combs = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));

    let i = 22;
    let j = 255;
    println!("{} {}", i, j);
    let combs = init_combinations(blockers[i]);
    let attacks = init_attacks(i, &combs);
    print_boards(&[combs[j], attacks[j]], None);
    println!("{}\n{}\n\n", bb_to_str(combs[j]), bb_to_str(attacks[j]));
}
