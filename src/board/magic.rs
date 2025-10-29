// seed: &mut u32

use crate::util::{del_bit, get_bit, pop_bit, set_bit};

fn search_for_magic(sq: usize) {
    let mut combs: Vec<u64> = vec![0; 256];
    
}

// todo: also precompute attack bit handles? not magic, just bitx, bity -> changes

fn init_attacks(sq: usize, combinations: &[u64; 256]) -> [u64; 256] {
    let mut attacks = [0; 256];
    for i in 0..256 {
        let mut bb = combinations[i];
        let mut alt;

        // up
        alt = false;
        for j in (i+6..36).step_by(6) {
            alt = !alt;
            if alt {
                if !get_bit(bb, j) {
                    break;
                }
            } else {
                if get_bit(bb, j) {
                    break;
                }

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

    attacks
}

fn init_combinations(bb: u64) -> [u64; 256] {
    let mut combs = [0; 256];
    for (i, comb) in combs.iter_mut().enumerate() {
        let mut mask = bb;
        let mut bit = 0;
        while mask != 0 {
            let sq = pop_bit(&mut mask);
            if i & (1 << bit) != 0 {
                set_bit(comb, sq);
            }
            bit += 1
        }
    }

    combs
}

fn init_blocker_boards() -> [u64; 36] {
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
