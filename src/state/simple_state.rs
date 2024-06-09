use crate::{utils::coord_2_block, Grid};

use super::{Fillable, State, TrackingCandidates};

#[derive(Clone)]
pub struct SimpleState {
    grid: Grid,    // 棋盘
    row: [[bool; 10]; 9],   // row[r][num] = 第r行是否存在数num
    col: [[bool; 10]; 9],   // 同理
    block: [[bool; 10]; 9], // 同理
}

impl From<Grid> for SimpleState {
    fn from(puzzle: Grid) -> Self {
        let mut res = Self {
            grid: puzzle,
            row: [[false; 10]; 9],
            col: [[false; 10]; 9],
            block: [[false; 10]; 9],
        };
        for r in 0..9 {
            for c in 0..9 {
                res.row[r][res.grid.0[r][c] as usize] = true;
                res.col[c][res.grid.0[r][c] as usize] = true;
                res.block[coord_2_block(r, c)][res.grid.0[r][c] as usize] = true;
            }
        }
        res
    }
}

impl State for SimpleState {
    fn cell_val(&self, r: usize, c: usize) -> i8 {
        self.grid.0[r][c]
    }

    fn is_cell_empty(&self, r: usize, c: usize) -> bool {
        self.grid.0[r][c] == 0
    }

    fn grid(&self) -> Grid {
        self.grid
    }
}

impl TrackingCandidates for SimpleState {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool {
        let b = coord_2_block(r, c);
        !self.row[r][num as usize] && !self.col[c][num as usize] && !self.block[b][num as usize]
    }
}

impl Fillable for SimpleState {
    fn fill_cell(&mut self, r: usize, c: usize, num: i8) {
        let b = coord_2_block(r, c);
        self.grid.0[r][c] = num;
        self.row[r][num as usize] = true;
        self.col[c][num as usize] = true;
        self.block[b][num as usize] = true;
    }

    fn unfill_cell(&mut self, r: usize, c: usize) {
        let b = coord_2_block(r, c);
        let num = self.grid.0[r][c];
        self.grid.0[r][c] = 0;
        self.row[r][num as usize] = false;
        self.col[c][num as usize] = false;
        self.block[b][num as usize] = false;
    }
}
