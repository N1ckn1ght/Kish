use once_cell::sync::Lazy;
use crate::{magic::{BLOCKER_BOARDS, MAGICS, MAGIC_MAPS, SOLUTIONS}, util::{pop_bit, print_boards, BLACK, WHITE}};


pub struct Board {
    pub bb:             u64,               // bitboard
    pub turn:           bool,              // true if black to move
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
        
    pub fn iter_moves(&self) -> MoveIter {
        let mask = if self.turn {
            BLACK
        } else {
            WHITE
        };
        MoveIter {
            bb: self.bb,
            pieces: self.bb & mask,
            attacks: 0,
            curr_piece: 0
        }
    }

    pub fn make_move(&mut self, mov: u64) {
        self.bb ^= mov;
        self.turn = !self.turn;
    }

    pub fn count_moves(&self) -> u8 {
        let mut cnt = 0;
        let mask = if self.turn {
            BLACK
        } else {
            WHITE
        };
        let mut pieces = self.bb & mask;
        while pieces != 0 {
            let piece = pop_bit(&mut pieces);
            let occupancies = self.bb & BLOCKER_BOARDS[piece];
            let attacks = Self::get_attacks(piece, occupancies);
            cnt += attacks.count_ones() as u8;
        }
        cnt
    }

    #[inline]
    pub fn get_attacks(sq: usize, occupancies: u64) -> u64 {
        let magic_index = occupancies.wrapping_mul(MAGICS[sq]) >> 56;
        MAGIC_MAPS[sq][magic_index as usize]
    }
}


pub struct MoveIter {
    bb: u64,
    pieces: u64,
    attacks: u64,
    curr_piece: usize
}

impl<'a> Iterator for MoveIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.attacks != 0 {
                let attack = pop_bit(&mut self.attacks);
                return Some(SOLUTIONS[self.curr_piece][attack]);
            }
            if self.pieces == 0 {
                return None;
            }
            self.curr_piece = pop_bit(&mut self.pieces);
            let occupancies = self.bb & BLOCKER_BOARDS[self.curr_piece];
            self.attacks = Board::get_attacks(self.curr_piece, occupancies);
        }
    }

    fn game_prep_next(&mut self) -> Option<Self::Item> {
        
    }
}
