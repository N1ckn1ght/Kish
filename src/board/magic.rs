// seed: &mut u32

use crate::util::{del_bit, get_bit, pop_bit, print_boards, set_bit};

fn search_for_magic(sq: usize) {
    let mut combs: Vec<u64> = vec![0; 256];
    
}

// solutions to attack
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
