use super::utils::{block_idx_2_coord, coord_2_block};

pub trait TrackingCandidates {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool;
    fn get_candidates_of(&mut self, r: usize, c: usize) -> &mut [bool; 10];

    fn candidate_cnt_for_grid_in_row(&self, r: usize, c: usize) -> i8;
    fn candidate_cnt_for_grid_in_col(&self, c: usize, r: usize) -> i8;
    fn candidate_cnt_for_grid_in_blk(&self, b: usize, bidx: usize) -> i8;

    fn grid_cnt_for_candidate_in_row(&self, r: usize, num: i8) -> i8;
    fn grid_cnt_for_candidate_in_col(&self, c: usize, num: i8) -> i8;
    fn grid_cnt_for_candidate_in_blk(&self, b: usize, num: i8) -> i8;
}

pub trait Fillable {
    fn fill_grid(&mut self, r: usize, c: usize, num: i8);
    fn unfill_grid(&mut self, r: usize, c: usize);
}

pub struct SudokuPuzzle {
    pub board: [[i8; 9]; 9],
    candidates: [[[bool; 10]; 9]; 9],
    candidate_cnt: [[i8; 9]; 9],
    pub grid_cnt_for_candidate_in_row: [[i8; 10]; 9],
    grid_cnt_for_candidate_in_col: [[i8; 10]; 9],
    grid_cnt_for_candidate_in_blk: [[i8; 10]; 9],
}

impl TrackingCandidates for SudokuPuzzle {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool {
        self.candidates[r][c][num as usize]
    }
    fn get_candidates_of(&mut self, r: usize, c: usize) -> &mut [bool; 10] {
        &mut self.candidates[r][c]
    }

    fn candidate_cnt_for_grid_in_row(&self, r: usize, c: usize) -> i8 {
        self.candidate_cnt[r][c]
    }
    fn candidate_cnt_for_grid_in_col(&self, c: usize, r: usize) -> i8 {
        self.candidate_cnt[r][c]
    }
    fn candidate_cnt_for_grid_in_blk(&self, b: usize, bidx: usize) -> i8 {
        let (r, c) = block_idx_2_coord(b, bidx);
        self.candidate_cnt[r][c]
    }

    fn grid_cnt_for_candidate_in_row(&self, r: usize, num: i8) -> i8 {
        self.grid_cnt_for_candidate_in_row[r][num as usize]
    }
    fn grid_cnt_for_candidate_in_col(&self, c: usize, num: i8) -> i8 {
        self.grid_cnt_for_candidate_in_col[c][num as usize]
    }
    fn grid_cnt_for_candidate_in_blk(&self, b: usize, num: i8) -> i8 {
        self.grid_cnt_for_candidate_in_blk[b][num as usize]
    }
}

impl Fillable for SudokuPuzzle {
    fn fill_grid(&mut self, r: usize, c: usize, num: i8) {
        self.board[r][c] = num;
        let num = num as usize;
        let b = coord_2_block(r, c);

        for c1 in 0..9 {
            self.candidate_cnt[r][c1] -= self.candidates[r][c1][num] as i8;
            self.candidates[r][c1][num] = false;
        }
        for r1 in 0..9 {
            self.candidate_cnt[r1][c] -= self.candidates[r1][c][num] as i8;
            self.candidates[r1][c][num] = false;
        }
        for bidx in 0..9 {
            let (r1, c1) = block_idx_2_coord(b, bidx);
            self.candidate_cnt[r1][c1] -= self.candidates[r1][c1][num] as i8;
            self.candidates[r1][c1][num] = false;
        }

        // 一旦填了一个新的数， grid_cnt 就需要全局更新
        for num1 in 1..=9 {
            if self.candidates[r][c][num1] {
                self.grid_cnt_for_candidate_in_row[r][num1] -= 1;
                self.grid_cnt_for_candidate_in_col[c][num1] -= 1;
                self.grid_cnt_for_candidate_in_blk[b][num1] -= 1;
            }
        }
        for r in 0..9 {
            let mut grid_cnt = 0;
            for c in 0..9 {
                grid_cnt += (self.candidates[r][c][num] && self.board[r][c] == 0) as i8;
            }
            self.grid_cnt_for_candidate_in_row[r][num] = grid_cnt;
        }
        for c in 0..9 {
            let mut grid_cnt = 0;
            for r in 0..9 {
                grid_cnt += (self.candidates[r][c][num] && self.board[r][c] == 0) as i8;
            }
            self.grid_cnt_for_candidate_in_col[c][num] = grid_cnt;
        }
        for b in 0..9 {
            let mut grid_cnt = 0;
            for bidx in 0..9 {
                let (r, c) = block_idx_2_coord(b, bidx);
                grid_cnt += (self.candidates[r][c][num] && self.board[r][c] == 0) as i8;
            }
            self.grid_cnt_for_candidate_in_blk[b][num] = grid_cnt;
        }
    }

    fn unfill_grid(&mut self, r: usize, c: usize) {
        let num = self.board[r][c] as usize;
        self.board[r][c] = 0;
        self.candidates[r][c] = [true; 10];
        for c1 in 0..9 {
            self.candidate_cnt[r][c1] += !self.candidates[r][c1][num] as i8;
            self.grid_cnt_for_candidate_in_row[r][num] += !self.candidates[r][c1][num] as i8;
            self.candidates[r][c1][num] = true;
            // let num1 = self.board[r][c1] as usize;
            // self.candidate_cnt[r][c] -= self.candidates[r][c][num1] as i8;
            // self.candidates[r][c][num1] = false;
        }
        for r1 in 0..9 {
            self.candidate_cnt[r1][c] += !self.candidates[r1][c][num] as i8;
            self.grid_cnt_for_candidate_in_col[c][num] += !self.candidates[r1][c][num] as i8;
            self.candidates[r1][c][num] = true;
            // let num1 = self.board[r1][c] as usize;
            // self.candidate_cnt[r][c] -= self.candidates[r][c][num1] as i8;
            // self.candidates[r][c][num1] = false;
        }
        let b = coord_2_block(r, c);
        for bidx in 0..9 {
            let (r1, c1) = block_idx_2_coord(b, bidx);
            self.candidate_cnt[r1][c1] += !self.candidates[r1][c1][num] as i8;
            self.grid_cnt_for_candidate_in_col[b][num] += !self.candidates[r1][c1][num] as i8;
            self.candidates[r1][c1][num] = true;
            // let num1 = self.board[r1][c1] as usize;
            // self.candidate_cnt[r][c] -= self.candidates[r][c][num1] as i8;
            // self.candidates[r][c][num1] = false;
        }
    }
}

impl SudokuPuzzle {
    pub fn new(puzzle: [[i8; 9]; 9]) -> Self {
        let mut res = Self {
            board: [[0; 9]; 9],
            candidates: [[[true; 10]; 9]; 9],
            candidate_cnt: [[9; 9]; 9],
            grid_cnt_for_candidate_in_row: [[9; 10]; 9],
            grid_cnt_for_candidate_in_col: [[9; 10]; 9],
            grid_cnt_for_candidate_in_blk: [[9; 10]; 9],
        };
        for r in 0..9 {
            for c in 0..9 {
                if puzzle[r][c] > 0 {
                    res.fill_grid(r, c, puzzle[r][c]);
                }
            }
        }
        res
    }

    pub fn grid_val(&self, r: usize, c: usize) -> i8 {
        self.board[r][c]
    }
}
