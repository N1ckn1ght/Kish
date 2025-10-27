// seed: &mut u32

use crate::util::set_bit;

fn search_for_magic(sq: usize) {
    let mut combs: Vec<u64> = vec![0; 256];
    
}

fn init_blocker_boards() -> [u64; 256] {
    let mut bbs = [0; 256];
    for (i, bb) in bbs.iter_mut().enumerate() {
        let mut buf = 0;
        // up
        for j in (i..36).step_by(6) {
            if buf == 0 {
                buf = j
            } else {
                set_bit(bb, j);
                set_bit(bb, buf);
                buf = 0;
            }
        }
        // right
        for j in i..((i / 6 + 1) * 6) {
            
        }

        for j in (0..i+1).rev().step_by(6) {
            
        }
    }

    bbs
}
