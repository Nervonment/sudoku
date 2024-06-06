use crate::utils::coord_2_block;

use super::{Fillable, State, TrackingCandidates};

pub struct SimpleState {
    board: [[i8; 9]; 9],    // 棋盘
    row: [[bool; 10]; 9],   // row[r][num] = 第r行是否存在数num
    col: [[bool; 10]; 9],   // 同理
    block: [[bool; 10]; 9], // 同理
}

impl From<[[i8; 9]; 9]> for SimpleState {
    fn from(puzzle: [[i8; 9]; 9]) -> Self {
        let mut res = Self {
            board: puzzle,
            row: [[false; 10]; 9],
            col: [[false; 10]; 9],
            block: [[false; 10]; 9],
        };
        for r in 0..9 {
            for c in 0..9 {
                res.row[r][res.board[r][c] as usize] = true;
                res.col[c][res.board[r][c] as usize] = true;
                res.block[coord_2_block(r, c)][res.board[r][c] as usize] = true;
            }
        }
        res
    }
}

impl State for SimpleState {
    fn grid_val(&self, r: usize, c: usize) -> i8 {
        self.board[r][c]
    }

    fn is_grid_empty(&self, r: usize, c: usize) -> bool {
        self.board[r][c] == 0
    }

    fn board(&self) -> [[i8; 9]; 9] {
        self.board
    }
}

impl TrackingCandidates for SimpleState {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool {
        let b = coord_2_block(r, c);
        !self.row[r][num as usize] && !self.col[c][num as usize] && !self.block[b][num as usize]
    }
}

impl Fillable for SimpleState {
    fn fill_grid(&mut self, r: usize, c: usize, num: i8) {
        let b = coord_2_block(r, c);
        self.board[r][c] = num;
        self.row[r][num as usize] = true;
        self.col[c][num as usize] = true;
        self.block[b][num as usize] = true;
    }

    fn unfill_grid(&mut self, r: usize, c: usize) {
        let b = coord_2_block(r, c);
        let num = self.board[r][c];
        self.board[r][c] = 0;
        self.row[r][num as usize] = false;
        self.col[c][num as usize] = false;
        self.block[b][num as usize] = false;
    }
}
