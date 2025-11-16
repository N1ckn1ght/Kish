use crate::{magic::{BLOCKER_BOARDS, MAGICS, MAGIC_MAPS, SOLUTIONS}, util::{pop_bit}};


pub const WHITE: u64 = 0b101010010101101010010101101010010101;
pub const BLACK: u64 = 0b010101101010010101101010010101101010;
pub const TURN: u64 = 1 << 36;


pub struct MoveIter {
    bb: u64,
    pieces: u64,  // pieces to move remaining
    attacks: u64,  // attacks remaining for current piece
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
            self.attacks = get_attacks(self.curr_piece, occupancies);
        }
    }

    fn pre_game_next(&mut self) -> Option<Self::Item> {
        
    }
}


#[inline]
pub fn make_move(bb: u64, mov: u64) -> u64 {
    bb ^ mov | TURN
}

#[inline]
pub fn get_attacks(sq: usize, occupancies: u64) -> u64 {
    let magic_index = occupancies.wrapping_mul(MAGICS[sq]) >> 56;
    MAGIC_MAPS[sq][magic_index as usize]
}

pub fn iter_moves(bb: u64) -> MoveIter {
    let mask = if bb & TURN != 0 {
        BLACK
    } else {
        WHITE
    };
    MoveIter {
        bb: bb,
        pieces: bb & mask,
        attacks: 0,
        curr_piece: 0
    }
}
