use crate::{maps::{BLOCKER_BOARDS, MAGICS, MAGIC_MAPS, NEIGHBOURS, SOLUTIONS}, util::pop_bit};

// main masks
pub const WHITE: u64 = 0b101010010101101010010101101010010101;
pub const BLACK: u64 = 0b010101101010010101101010010101101010;

// additional bits
pub const TURN: u64 = 1 << 36;
pub const FMOV: u64 = 1 << 37;  // first move bit, stage where players need to remove 1 piece of their color

// copy this
pub const NEW_BOARD: u64 = 0b10111111111111111111111111111111111111;


pub struct MoveIter {
    bb: u64,
    pieces: u64,  // pieces to move remaining
    attacks: u64,  // attacks remaining for current piece
    curr_piece: usize
}

impl<'a> Iterator for MoveIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bb & FMOV != 0 {
            // if first full move
            loop {
                if self.attacks != 0 {
                    let solution = pop_bit(&mut self.attacks);
                    let fmov_bit_removal = self.bb & TURN << 1;
                    return Some(1 << solution | fmov_bit_removal);
                }
                // self.pieces used here just as a flag, we don't need them
                if self.pieces != 0 {
                    return None;
                }
                self.pieces = FMOV;
                if self.bb & TURN != 0 {
                    self.attacks = NEIGHBOURS[self.bb.trailing_ones() as usize];
                } else {
                    self.attacks = WHITE;       
                }
            }
        } else {
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