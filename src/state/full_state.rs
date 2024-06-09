use crate::{
    utils::{block_idx_2_coord, coord_2_block},
    Grid,
};

use super::{
    CandidatesSettable, Fillable, State, TrackingCandidateCountOfCell, TrackingCandidates,
    TrackingCellCountOfCandidate,
};

#[derive(Clone)]
pub struct FullState {
    grid: Grid,
    candidates: [[[bool; 10]; 9]; 9],
    candidate_cnt: [[i8; 9]; 9],
    cell_cnt_for_candidate_in_row: [[i8; 10]; 9],
    cell_cnt_for_candidate_in_col: [[i8; 10]; 9],
    cell_cnt_for_candidate_in_blk: [[i8; 10]; 9],
    history: Vec<(
        [[[bool; 10]; 9]; 9],
        [[i8; 9]; 9],
        [[i8; 10]; 9],
        [[i8; 10]; 9],
        [[i8; 10]; 9],
    )>,
}

impl FullState {
    pub fn new(grid: Grid, candidates: [[[bool; 10]; 9]; 9]) -> Self {
        let mut res = Self {
            grid,
            candidates: [[[true; 10]; 9]; 9],
            candidate_cnt: [[9; 9]; 9],
            cell_cnt_for_candidate_in_row: [[9; 10]; 9],
            cell_cnt_for_candidate_in_col: [[9; 10]; 9],
            cell_cnt_for_candidate_in_blk: [[9; 10]; 9],
            history: vec![],
        };
        for r in 0..9 {
            for c in 0..9 {
                for num in 1..=9 {
                    if !candidates[r][c][num as usize] {
                        res.remove_candidate_of_cell(r, c, num)
                    }
                }
            }
        }
        res
    }
}

impl From<Grid> for FullState {
    fn from(puzzle: Grid) -> Self {
        let mut res = Self {
            grid: Grid([[0; 9]; 9]),
            candidates: [[[true; 10]; 9]; 9],
            candidate_cnt: [[9; 9]; 9],
            cell_cnt_for_candidate_in_row: [[9; 10]; 9],
            cell_cnt_for_candidate_in_col: [[9; 10]; 9],
            cell_cnt_for_candidate_in_blk: [[9; 10]; 9],
            history: vec![],
        };
        for r in 0..9 {
            for c in 0..9 {
                if puzzle.0[r][c] > 0 {
                    res.fill_cell(r, c, puzzle.0[r][c]);
                }
            }
        }
        res
    }
}

impl State for FullState {
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

impl TrackingCandidates for FullState {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool {
        self.candidates[r][c][num as usize]
    }
}

impl TrackingCandidateCountOfCell for FullState {
    fn candidate_cnt_of_cell(&self, r: usize, c: usize) -> i8 {
        self.candidate_cnt[r][c]
    }
    fn candidate_cnt_of_cell_in_row(&self, r: usize, c: usize) -> i8 {
        self.candidate_cnt[r][c]
    }
    fn candidate_cnt_of_cell_in_col(&self, c: usize, r: usize) -> i8 {
        self.candidate_cnt[r][c]
    }
    fn candidate_cnt_of_cell_in_blk(&self, b: usize, bidx: usize) -> i8 {
        let (r, c) = block_idx_2_coord(b, bidx);
        self.candidate_cnt[r][c]
    }
}

impl TrackingCellCountOfCandidate for FullState {
    fn cell_cnt_of_candidate_in_row(&self, r: usize, num: i8) -> i8 {
        self.cell_cnt_for_candidate_in_row[r][num as usize]
    }
    fn cell_cnt_of_candidate_in_col(&self, c: usize, num: i8) -> i8 {
        self.cell_cnt_for_candidate_in_col[c][num as usize]
    }
    fn cell_cnt_of_candidate_in_blk(&self, b: usize, num: i8) -> i8 {
        self.cell_cnt_for_candidate_in_blk[b][num as usize]
    }
}

impl Fillable for FullState {
    // 在格 (r, c) 处填上 num
    fn fill_cell(&mut self, r: usize, c: usize, num: i8) {
        // 记录历史状态
        self.history.push((
            self.candidates,
            self.candidate_cnt,
            self.cell_cnt_for_candidate_in_row,
            self.cell_cnt_for_candidate_in_col,
            self.cell_cnt_for_candidate_in_blk,
        ));

        self.grid.0[r][c] = num;
        let num = num as usize;
        let b = coord_2_block(r, c);

        // 更新 candidates 和 candidate_cnt
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

        // 更新 grid_cnt_for_candidate_in_xxx

        // 因为当前格子被填上，所以它所在行的候选数包括 num1 的格子数都要减少1，
        // 其中 num1 是当前格子的所有候选数
        for num1 in 1..=9 {
            if self.candidates[r][c][num1] {
                self.cell_cnt_for_candidate_in_row[r][num1] -= 1;
                self.cell_cnt_for_candidate_in_col[c][num1] -= 1;
                self.cell_cnt_for_candidate_in_blk[b][num1] -= 1;
            }
        }
        // 对于所有行、所有列、9个宫其中的5个（这里为了行文方便，所有宫都更新了），
        // 它们中的候选数包括所填数 num 的格子的数量都可能发生变化，需要更新
        for r in 0..9 {
            let mut cell_cnt = 0;
            for c in 0..9 {
                cell_cnt += (self.candidates[r][c][num] && self.grid.0[r][c] == 0) as i8;
            }
            self.cell_cnt_for_candidate_in_row[r][num] = cell_cnt;
        }
        for c in 0..9 {
            let mut cell_cnt = 0;
            for r in 0..9 {
                cell_cnt += (self.candidates[r][c][num] && self.grid.0[r][c] == 0) as i8;
            }
            self.cell_cnt_for_candidate_in_col[c][num] = cell_cnt;
        }
        for b in 0..9 {
            let mut cell_cnt = 0;
            for bidx in 0..9 {
                let (r, c) = block_idx_2_coord(b, bidx);
                cell_cnt += (self.candidates[r][c][num] && self.grid.0[r][c] == 0) as i8;
            }
            self.cell_cnt_for_candidate_in_blk[b][num] = cell_cnt;
        }
    }

    // 撤销上一步填充
    fn unfill_cell(&mut self, r: usize, c: usize) {
        // 回退
        self.grid.0[r][c] = 0;
        (
            self.candidates,
            self.candidate_cnt,
            self.cell_cnt_for_candidate_in_row,
            self.cell_cnt_for_candidate_in_col,
            self.cell_cnt_for_candidate_in_blk,
        ) = self.history.pop().unwrap();
    }
}

impl CandidatesSettable for FullState {
    fn remove_candidate_of_cell(&mut self, r: usize, c: usize, to_remove: i8) {
        let to_remove = to_remove as usize;
        let b = coord_2_block(r, c);
        let is_candidate = self.candidates[r][c][to_remove];
        self.candidate_cnt[r][c] -= is_candidate as i8;
        self.cell_cnt_for_candidate_in_row[r][to_remove] -= is_candidate as i8;
        self.cell_cnt_for_candidate_in_col[c][to_remove] -= is_candidate as i8;
        self.cell_cnt_for_candidate_in_blk[b][to_remove] -= is_candidate as i8;
        self.candidates[r][c][to_remove] = false;
    }
    fn add_candidate_of_cell(&mut self, r: usize, c: usize, to_add: i8) {
        let to_add = to_add as usize;
        let b = coord_2_block(r, c);
        let is_candidate = self.candidates[r][c][to_add];
        self.candidate_cnt[r][c] += !is_candidate as i8;
        self.cell_cnt_for_candidate_in_row[r][to_add] += !is_candidate as i8;
        self.cell_cnt_for_candidate_in_col[c][to_add] += !is_candidate as i8;
        self.cell_cnt_for_candidate_in_blk[b][to_add] += !is_candidate as i8;
        self.candidates[r][c][to_add] = true;
    }
}
