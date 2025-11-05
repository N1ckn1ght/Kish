use once_cell::sync::Lazy;

static BLOCKER_BOARDS: Lazy<[u64; 36]> = Lazy::new(init_blocker_boards);


pub struct Board {
    pub bb:             u64,               // bitboard
    // pub hmc:            u8,                // halfmove clock
    pub turn:           bool,              // true for black
    // pub blocker_boards: [u64; 36],
    // pub combinations:   [[u64; 256]; 36],
    // pub attacks:        [[u64; 256]; 36],
    // pub solutions:      [[u64; 36]; 36],
    // pub magics:         [u64; 36]
}

impl Default for Board {
    fn default() -> Board {

    }
}

impl Board {
    pub fn import(description: &str) -> Self {
        // oxoxox/xoxoxo/oxoxox/xo-oxo/oxox-x/xoxoxo
        // xxxxxx/xxxxxx/xxxxxx/xx-xxx/xxxx-x/xxxxxx
        
    }

    pub fn import_from_moves(moves: &str) -> Self {
        // e3 e2 c3e3
        
    }
}