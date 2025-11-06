use once_cell::sync::Lazy;
use crate::magic::{init_attacks, init_blocker_boards, init_combinations, init_magics, init_solutions};

pub static BLOCKER_BOARDS: Lazy<[u64; 36]> = Lazy::new(init_blocker_boards);
pub static COMBINATIONS: Lazy<[[u64; 256]; 36]> = Lazy::new(|| init_combinations(&BLOCKER_BOARDS));
pub static ATTACKS: Lazy<[[u64; 256]; 36]> = Lazy::new(|| init_attacks(&COMBINATIONS));
pub static SOLUTIONS: Lazy<[[u64; 36]; 36]> = Lazy::new(init_solutions);
pub static MAGICS: Lazy<[u64; 36]> = Lazy::new(|| init_magics(1, &COMBINATIONS, &ATTACKS));


pub struct Board {
    pub bb:             u64,               // bitboard
    pub turn:           bool,              // true for black
}

impl Default for Board {
    fn default() -> Board {
        Board {
            bb: 0xFFFFFFFFF,
            turn: false
        }
    }
}

impl Board {
    // pub fn import(description: &str) -> Self {
    //     // oxoxox/xoxoxo/oxoxox/xo-oxo/oxox-x/xoxoxo
    //     // xxxxxx/xxxxxx/xxxxxx/xx-xxx/xxxx-x/xxxxxx
        
    // }

    // pub fn import_from_moves(moves: &str) -> Self {
    //     // e3 e2 c3e3
        
    // }
}