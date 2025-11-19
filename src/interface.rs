use std::collections::HashMap;
use crate::board::{make_move, BLACK, NEW_BOARD, WHITE};

pub static ERR_MV: u64 = 1 << 63;


pub struct Interface {
    board: u64,
    moves: HashMap<String, u64>,
    ready: bool
}

impl Interface {
    pub fn init() -> Self {
        Self {
            board: NEW_BOARD,
            moves: HashMap::new(),
            ready: false
        }
    }

    pub fn clear(&mut self) {
        self.board = NEW_BOARD;
        self.ready = false;
    }

    pub fn gen_moves(&mut self) {
        
        // self.ready = true;
    }

    pub fn make_move(&mut self, mov_str: &str) -> bool {
        if self.ready {
            let mov = self.encode_move(mov_str);
            self.board = make_move(self.board, mov);
            self.ready = false;
            return true;
        }
        false
    }

    pub fn encode_move(&self, mov_str: &str) -> u64 {
        let value = self.moves.get(mov_str);
        match value {
            Some(mov) => return *mov,
            None => return ERR_MV
        }
    }

    pub fn decode_move(&self, mut mov: u64) -> String {
        if mov & ERR_MV != 0 {
            return String::from("Illegal move");
        }
        // if first move bit
        mov &= WHITE | BLACK;
        let file = (mov % 6) as u8;
        let rank = (mov / 6 + 1) as u8;
        format!("{}{}", (b'a' + file) as char, rank)
        // else
        // take lowest and highest as coordinates from and to, consider where's the piece
    }
}