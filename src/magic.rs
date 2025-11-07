use once_cell::sync::Lazy;
use crate::util::{bb_to_str, del_bit, get_bit, pop_bit, print_boards, set_bit};

pub static BLOCKER_BOARDS: Lazy<[u64; 36]> = Lazy::new(init_blocker_boards);
pub static COMBINATIONS: Lazy<[[u64; 256]; 36]> = Lazy::new(|| init_combinations(&BLOCKER_BOARDS));
pub static ATTACKS: Lazy<[[u64; 256]; 36]> = Lazy::new(|| init_attacks(&COMBINATIONS));
pub static SOLUTIONS: Lazy<[[u64; 36]; 36]> = Lazy::new(init_solutions);
pub static MAGICS: Lazy<[u64; 36]> = Lazy::new(|| init_magics(1, &COMBINATIONS, &ATTACKS));
pub static MAGIC_MAPS: Lazy<[[u64; 256]; 36]> = Lazy::new(|| init_magic_maps(&COMBINATIONS, &ATTACKS, &MAGICS));


pub fn init_magic_maps(combs: &[[u64; 256]; 36], attacks: &[[u64; 256]; 36], magics: &[u64; 36]) -> [[u64; 256]; 36] {
    let mut magic_maps = [[0; 256]; 36];
    for sq in 0..36 {
        for i in 0..256 {
            let index = (combs[sq][i].wrapping_mul(magics[sq]) >> 56) as usize;
            magic_maps[sq][index] = attacks[sq][i];
        }
    }
    magic_maps
}

pub fn init_magics(mut seed: u64, combs: &[[u64; 256]; 36], attacks: &[[u64; 256]; 36]) -> [u64; 36] {
    let mut magics = [0; 36];
    for i in 0..36 {
        magics[i] = search_for_magic(i, 8, &combs[i], &attacks[i], &mut seed, 1048576);
    }
    magics
}

// solutions to attacks
#[allow(clippy::needless_range_loop)]
pub fn init_solutions() -> [[u64; 36]; 36] {
    let mut solutions = [[0; 36]; 36];
    for from in 0..36 {
        let mut alt;

        // up
        alt = false;
        let mut mask = 1 << from;
        for to in (from+6..36).step_by(6) {
            alt = !alt;
            if alt {
                set_bit(&mut mask, to);
            } else {
                set_bit(&mut mask, to);
                solutions[from][to] = mask;
                del_bit(&mut mask, to);
            }
        }

        // right
        alt = false;
        let mut mask = 1 << from;
        for to in from+1..((from / 6 + 1) * 6) {
            alt = !alt;
            if alt {
                set_bit(&mut mask, to);
            } else {
                set_bit(&mut mask, to);
                solutions[from][to] = mask;
                del_bit(&mut mask, to);
            }
        }

        // down
        if from > 11 {
            alt = false;
            let mut mask = 1 << from;
            for to in (0..from-5).rev().step_by(6) {
                alt = !alt;
                if alt {
                    set_bit(&mut mask, to);
                } else {
                    set_bit(&mut mask, to);
                    solutions[from][to] = mask;
                    del_bit(&mut mask, to);
                }
            }
        }

        // left
        alt = false;
        let mut mask = 1 << from;
        for to in (from/6*6..from).rev() {
            alt = !alt;
            if alt {
                set_bit(&mut mask, to);
            } else {
                set_bit(&mut mask, to);
                solutions[from][to] = mask;
                del_bit(&mut mask, to);
            }
        }
    }

    solutions
}

pub fn init_attacks(combinations: &[[u64; 256]; 36]) -> [[u64; 256]; 36] {
    let mut attacks = [[0; 256]; 36];
    for sq in 0..36 {
        for i in 0..256 {
            let bb = combinations[sq][i];
            let mut alt;

            // up
            alt = false;
            for j in (sq+6..36).step_by(6) {
                alt = !alt;
                if alt {
                    if get_bit(bb, j) == 0 {
                        break;
                    }
                } else {
                    if get_bit(bb, j) != 0 {
                        break;
                    }
                    set_bit(&mut attacks[sq][i], j);
                }
            }

            // right
            alt = false;
            for j in sq+1..((sq / 6 + 1) * 6) {
                alt = !alt;
                if alt {
                    if get_bit(bb, j) == 0 {
                        break;
                    }
                } else {
                    if get_bit(bb, j) != 0 {
                        break;
                    }
                    set_bit(&mut attacks[sq][i], j);
                }
            }

            // down
            if sq > 11 {
                alt = false;
                for j in (0..sq-5).rev().step_by(6) {
                    alt = !alt;
                    if alt {
                        if get_bit(bb, j) == 0 {
                            break;
                        }
                    } else {
                        if get_bit(bb, j) != 0 {
                            break;
                        }
                        set_bit(&mut attacks[sq][i], j);
                    }
                }
            }

            // left
            alt = false;
            for j in (sq/6*6..sq).rev() {
                alt = !alt;
                if alt {
                    if get_bit(bb, j) == 0 {
                        break;
                    }
                } else {
                    if get_bit(bb, j) != 0 {
                        break;
                    }
                    set_bit(&mut attacks[sq][i], j);
                }
            }
        }
    }
    
    attacks
}

pub fn init_combinations(bbs: &[u64; 36]) -> [[u64; 256]; 36] {
    let mut combs = [[0; 256]; 36];
    for (sq, bb) in bbs.iter().enumerate() {
        for (i, comb) in combs[sq].iter_mut().enumerate() {
            let mut mask = *bb;
            let mut bit = 0;
            while mask != 0 {
                let csq = pop_bit(&mut mask);
                if i & (1 << bit) != 0 {
                    set_bit(comb, csq);
                }
                bit += 1
            }
        }
    }

    combs
}

pub fn init_blocker_boards() -> [u64; 36] {
    let mut bbs = [0; 36];
    for (i, bb) in bbs.iter_mut().enumerate() {
        let mut buf;

        // up
        buf = 0;
        for j in (i+6..36).step_by(6) {
            if buf == 0 {
                buf = j;
            } else {
                set_bit(bb, j);
                set_bit(bb, buf);
                buf = 0;
            }
        }

        // right
        buf = 0;
        for j in i+1..((i / 6 + 1) * 6) {
            if buf == 0 {
                buf = j;
            } else {
                set_bit(bb, j);
                set_bit(bb, buf);
                buf = 0;
            }
        }

        // down
        if i > 11 {
            buf = 0;
            for j in (0..i-5).rev().step_by(6) {
                if buf == 0 {
                    buf = j;
                } else {
                    set_bit(bb, j);
                    set_bit(bb, buf);
                    buf = 0;
                }
            }
        }

        // left
        buf = 0;
        for j in (i/6*6..i).rev() {
            if buf == 0 {
                buf = j;
            } else {
                set_bit(bb, j);
                set_bit(bb, buf);
                buf = 0;
            }
        }

        del_bit(bb, i);
    }

    bbs
}

fn search_for_magic(sq: usize, target: usize, combs: &[u64; 256], attacks: &[u64; 256], seed: &mut u64, limit: usize) -> u64 {
    let mut magic = 0;
    let mut fail = true;
    for attempt in 0..limit {
        magic = next_random_magic(seed);
        fail = false;
        let mut used = vec![0; 1 << target];
        for i in 0..256 {
            let index = (combs[i].wrapping_mul(magic) >> (64 - target)) as usize;
            // println!("{} {}", bb_to_str(index.try_into().unwrap()), attacks[i]);
            if used[index] != 0 && used[index] != attacks[i] {
                fail = true;
                break;
            }
            used[index] = attacks[i];
        }
        if !fail {
            println!("#DEBUG\tFound magic: sq {}\ttry {}", sq, attempt);
            break;
        }
    }
    if fail {
        panic!("Unable to find magic: sq {}\ttry {}\nlast seed {}\nlast magic {}", sq, limit, seed, magic);
    }
    magic
}

fn next_random_magic(seed: &mut u64) -> u64 {
    *seed = xor64(*seed);
    let mut magic = *seed;
    *seed = xor64(*seed);
    magic &= *seed;
    *seed = xor64(*seed);
    magic &= *seed;
    magic
}

fn xor64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

pub fn print_attacks(square: usize, pattern: usize) {
    let blockers = init_blocker_boards();
    let combinations = init_combinations(&blockers);
    let attacks = init_attacks(&combinations);
    let solutions = init_solutions();

    let i = square;
    let j = pattern;
    println!("{} {}", i, j);
    print_boards(&[combinations[i][j], attacks[i][j]], None);
    let mut solves = vec![];
    let mut attack = attacks[i][j];
    while attack != 0 {
        let bit = pop_bit(&mut attack);
        solves.push(solutions[i][bit]);
    }
    if !solves.is_empty() {
        print_boards(&solves, None);
    }
    println!("{}\n{}\n", bb_to_str(combinations[i][j]), bb_to_str(attacks[i][j]));
    for solve in solves.iter() {
        println!("{}", bb_to_str(*solve));
    }
    println!("\n");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_init() {
        let blockers = init_blocker_boards();
        let combinations = init_combinations(&blockers);
        let attacks = init_attacks(&combinations);

        // square (0 - 35) - pattern No. (0 - 255) pairs
        let inputs = [
            [7, 85],
            [35, 90],
            [35, 165],
            [15, 157],
            [15, 185],
            [22, 106],
            [22, 157],
            [22, 185],
            [22, 255]
        ];

        // expected results
        let combs: [u64; 9] = [
            0b000000000010000000000010010100000000,
            0b001010100000000000100000000000000000,
            0b010100000000100000000000100000000000,
            0b000000001000000000010110000000001000,
            0b000000001000000000110100000000001000,
            0b000000010000001010010000000000000000,
            0b010000000000000111000000010000000000,
            0b010000000000001110000000010000000000,
            0b010000010000001111010000010000000000
        ];
        let attack: [u64; 9] = [
            0b000010000000000010000000101000000000,
            0b000000000000100000000000100000000000,
            0b001010000000000000000000000000000000,
            0b000000000000000000100000000000000000,
            0b000000000000000000000010000000000000,
            0b010000000000000101000000010000000000,
            0b000000000000000000000000000000000000,
            0b000000000000000000000000000000000000,
            0b000000000000000000000000000000000000
        ];

        for (i, input) in inputs.iter().enumerate() {
            assert_eq!(combinations[input[0]][input[1]], combs[i]);
            assert_eq!(attacks[input[0]][input[1]], attack[i]);
        }
    }

    #[test]
    fn test_magic_solutions() {
        let solutions = init_solutions();

        let mut solves = vec![];
        let sq = 7;
        let mut attack = 0b000010000000000010000000101000000000;
        while attack != 0 {
            let bit = pop_bit(&mut attack);
            solves.push(solutions[sq][bit]);
        }
        assert_eq!(solves.len(), 4);
        assert_eq!(solves[0], 0b000000000000000000000000001110000000);
        assert_eq!(solves[1], 0b000000000000000000000000110110000000);
        assert_eq!(solves[2], 0b000000000000000010000010000010000000);
        assert_eq!(solves[3], 0b000010000010000000000010000010000000);

        let mut solves = vec![];
        let sq = 35;
        let mut attack = 0b000000000000100000000000100000000000;
        while attack != 0 {
            let bit = pop_bit(&mut attack);
            solves.push(solutions[sq][bit]);
        }
        assert_eq!(solves.len(), 2);
        assert_eq!(solves[0], 0b100000100000000000100000100000000000);
        assert_eq!(solves[1], 0b100000100000100000000000000000000000);

        let mut solves = vec![];
        let sq = 35;
        let mut attack = 0b001010000000000000000000000000000000;
        while attack != 0 {
            let bit = pop_bit(&mut attack);
            solves.push(solutions[sq][bit]);
        }
        assert_eq!(solves.len(), 2);
        assert_eq!(solves[0], 0b110110000000000000000000000000000000);
        assert_eq!(solves[1], 0b111000000000000000000000000000000000);

        let mut solves = vec![];
        let sq = 15;
        let mut attack = 0b000000000000000000100000000000000000;
        while attack != 0 {
            let bit = pop_bit(&mut attack);
            solves.push(solutions[sq][bit]);
        }
        assert_eq!(solves.len(), 1);
        assert_eq!(solves[0], 0b000000000000000000111000000000000000);

        let mut solves = vec![];
        let sq = 15;
        let mut attack = 0b000000000000000000000010000000000000;
        while attack != 0 {
            let bit = pop_bit(&mut attack);
            solves.push(solutions[sq][bit]);
        }
        assert_eq!(solves.len(), 1);
        assert_eq!(solves[0], 0b000000000000000000001110000000000000);

        let mut solves = vec![];
        let sq = 22;
        let mut attack = 0b010000000000000101000000010000000000;
        while attack != 0 {
            let bit = pop_bit(&mut attack);
            solves.push(solutions[sq][bit]);
        }
        assert_eq!(solves.len(), 4);
        assert_eq!(solves[0], 0b000000000000010000010000010000000000);
        assert_eq!(solves[1], 0b000000000000011011000000000000000000);
        assert_eq!(solves[2], 0b000000000000011100000000000000000000);
        assert_eq!(solves[3], 0b010000010000010000000000000000000000);

        let mut solves = vec![];
        let sq = 22;
        let mut attack = 0b000000000000000000000000000000000000;
        while attack != 0 {
            let bit = pop_bit(&mut attack);
            solves.push(solutions[sq][bit]);
        }
        assert_eq!(solves.len(), 0);

        let mut solves = vec![];
        let sq = 15;
        let mut attack = 0b010000000000000101000000010000000000;
        while attack != 0 {
            let bit = pop_bit(&mut attack);
            solves.push(solutions[sq][bit]);
        }
        assert_eq!(solves.len(), 4);
        assert_eq!(solves[0], 0);
        assert_eq!(solves[1], 0);
        assert_eq!(solves[2], 0);
        assert_eq!(solves[3], 0);
    }
}