const WHITE: u64 = 0b101010010101101010010101101010010101;
const BLACK: u64 = 0b010101101010010101101010010101101010;


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

pub fn print_bitboard(bitboard: u64) {
    for rank in (0..6).rev() {
        for file in 0..6 {
            print!("{} ", (bitboard >> (rank * 6 + file)) & 1 );
        }
        println!();
    }
}

pub fn bb_to_str(bitboard: u64) -> String {
    format!("{bitboard:036b}")
}
