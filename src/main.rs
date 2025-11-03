use board::magic::*;
use util::{bb_to_str, print_boards};

use crate::util::pop_bit;

mod board;
mod util;

fn main() {
    println!("Hello, world!");

    let blockers = init_blocker_boards();
    let combinations = init_combinations(&blockers);
    let attacks = init_attacks(&combinations);
    let solutions = init_solutions();

    let i = 0;
    let j = 100;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));

    let i = 7;
    let j = 85;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));

    let i = 35;
    let j = 90;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));

    let i = 35;
    let j = 165;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));

    let i = 15;
    let j = 157;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));

    let i = 15;
    let j = 185;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));

    let i = 22;
    let j = 106;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));

    let i = 22;
    let j = 157;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));

    let i = 22;
    let j = 185;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));

    let i = 22;
    let j = 255;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if solves.len() > 0 {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));
}
