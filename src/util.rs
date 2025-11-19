use std::cmp::min;
use crate::board::WHITE;


// main operations

#[inline]
pub fn get_bit(bitboard: u64, bit: usize) -> u64 {
    bitboard & (1 << bit)
}

#[inline]
pub fn set_bit(bitboard: &mut u64, bit: usize) {
    *bitboard |= 1 << bit;
}

#[inline]
pub fn del_bit(bitboard: &mut u64, bit: usize) {
    *bitboard &= !(1 << bit);
}

#[inline]
pub fn gtz(bitboard: u64) -> usize {
    u64::trailing_zeros(bitboard) as usize
}

// return trailing zeros, then remove last bit
#[inline]
pub fn pop_bit(bitboard: &mut u64) -> usize {
    let bit = gtz(*bitboard);
    *bitboard &= *bitboard - 1;
    bit
}


// other aappliances

pub fn bb_to_str(bitboard: u64) -> String {
    format!("{bitboard:036b}")
}

pub fn print_boards(bbs: &[u64]) {
    let columns = 4;
    for i in (0..bbs.len()).step_by(columns){
        for r in (0..=30).rev().step_by(6) {
            for j in 0..min(bbs.len() - i, columns) {
                for f in r..r+6 {
                    let bit = get_bit(bbs[i + j], f);
                    if bit != 0 {
                        if bit & WHITE != 0 {
                            print!("X");
                        } else {
                            print!("O");
                        }
                    } else {
                        print!(".");
                    }
                }
                print!(" ");
            }
            println!();
        }
        println!();
    }
}