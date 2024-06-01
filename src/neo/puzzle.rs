use super::utils::{block_idx_2_coord, coord_2_block};

pub trait Grid {
    fn new(puzzle: [[i8; 9]; 9]) -> Self;
    fn grid_val(&self, r: usize, c: usize) -> i8;
    fn is_grid_empty(&self, r: usize, c: usize) -> bool;
    fn board(&self) -> [[i8; 9]; 9];
}

pub trait TrackingCandidates {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool;
}

pub trait TrackingCandidateCountOfGrid {
    fn candidate_cnt_of_grid(&self, r: usize, c: usize) -> i8;
    fn candidate_cnt_of_grid_in_row(&self, r: usize, c: usize) -> i8;
    fn candidate_cnt_of_grid_in_col(&self, c: usize, r: usize) -> i8;
    fn candidate_cnt_of_grid_in_blk(&self, b: usize, bidx: usize) -> i8;
}

pub trait TrackingGridCountOfCandidate {
    fn grid_cnt_of_candidate_in_row(&self, r: usize, num: i8) -> i8;
    fn grid_cnt_of_candidate_in_col(&self, c: usize, num: i8) -> i8;
    fn grid_cnt_of_candidate_in_blk(&self, b: usize, num: i8) -> i8;
}

pub trait Fillable {
    fn fill_grid(&mut self, r: usize, c: usize, num: i8);
    fn unfill_grid(&mut self, r: usize, c: usize);
}

pub trait CandidatesSettable {
    fn remove_candidate_of_grid(&mut self, r: usize, c: usize, to_remove: i8);
    fn add_candidate_of_grid(&mut self, r: usize, c: usize, to_add: i8);
}

pub struct SudokuPuzzleSimple {
    board: [[i8; 9]; 9],    // 棋盘
    row: [[bool; 10]; 9],   // row[r][num] = 第r行是否存在数num
    col: [[bool; 10]; 9],   // 同理
    block: [[bool; 10]; 9], // 同理
}

impl Grid for SudokuPuzzleSimple {
    fn new(puzzle: [[i8; 9]; 9]) -> Self {
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

impl TrackingCandidates for SudokuPuzzleSimple {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool {
        let b = coord_2_block(r, c);
        !self.row[r][num as usize] && !self.col[c][num as usize] && !self.block[b][num as usize]
    }
}

impl Fillable for SudokuPuzzleSimple {
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

pub struct SudokuPuzzleFull {
    board: [[i8; 9]; 9],
    candidates: [[[bool; 10]; 9]; 9],
    candidate_cnt: [[i8; 9]; 9],
    grid_cnt_for_candidate_in_row: [[i8; 10]; 9],
    grid_cnt_for_candidate_in_col: [[i8; 10]; 9],
    grid_cnt_for_candidate_in_blk: [[i8; 10]; 9],
    history: Vec<(
        [[[bool; 10]; 9]; 9],
        [[i8; 9]; 9],
        [[i8; 10]; 9],
        [[i8; 10]; 9],
        [[i8; 10]; 9],
    )>,
}

impl TrackingCandidates for SudokuPuzzleFull {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool {
        self.candidates[r][c][num as usize]
    }
}

impl TrackingCandidateCountOfGrid for SudokuPuzzleFull {
    fn candidate_cnt_of_grid(&self, r: usize, c: usize) -> i8 {
        self.candidate_cnt[r][c]
    }
    fn candidate_cnt_of_grid_in_row(&self, r: usize, c: usize) -> i8 {
        self.candidate_cnt[r][c]
    }
    fn candidate_cnt_of_grid_in_col(&self, c: usize, r: usize) -> i8 {
        self.candidate_cnt[r][c]
    }
    fn candidate_cnt_of_grid_in_blk(&self, b: usize, bidx: usize) -> i8 {
        let (r, c) = block_idx_2_coord(b, bidx);
        self.candidate_cnt[r][c]
    }
}

impl TrackingGridCountOfCandidate for SudokuPuzzleFull {
    fn grid_cnt_of_candidate_in_row(&self, r: usize, num: i8) -> i8 {
        self.grid_cnt_for_candidate_in_row[r][num as usize]
    }
    fn grid_cnt_of_candidate_in_col(&self, c: usize, num: i8) -> i8 {
        self.grid_cnt_for_candidate_in_col[c][num as usize]
    }
    fn grid_cnt_of_candidate_in_blk(&self, b: usize, num: i8) -> i8 {
        self.grid_cnt_for_candidate_in_blk[b][num as usize]
    }
}

impl Fillable for SudokuPuzzleFull {
    // 在格 (r, c) 处填上 num
    fn fill_grid(&mut self, r: usize, c: usize, num: i8) {
        // 记录历史状态
        self.history.push((
            self.candidates,
            self.candidate_cnt,
            self.grid_cnt_for_candidate_in_row,
            self.grid_cnt_for_candidate_in_col,
            self.grid_cnt_for_candidate_in_blk,
        ));

        self.board[r][c] = num;
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
                self.grid_cnt_for_candidate_in_row[r][num1] -= 1;
                self.grid_cnt_for_candidate_in_col[c][num1] -= 1;
                self.grid_cnt_for_candidate_in_blk[b][num1] -= 1;
            }
        }
        // 对于所有行、所有列、9个宫其中的5个（这里为了行文方便，所有宫都更新了），
        // 它们中的候选数包括所填数 num 的格子的数量都可能发生变化，需要更新
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

    // 撤销上一步填充
    fn unfill_grid(&mut self, r: usize, c: usize) {
        // 回退
        self.board[r][c] = 0;
        (
            self.candidates,
            self.candidate_cnt,
            self.grid_cnt_for_candidate_in_row,
            self.grid_cnt_for_candidate_in_col,
            self.grid_cnt_for_candidate_in_blk,
        ) = self.history.pop().unwrap();
    }
}

impl CandidatesSettable for SudokuPuzzleFull {
    fn remove_candidate_of_grid(&mut self, r: usize, c: usize, to_remove: i8) {
        let to_remove = to_remove as usize;
        let b = coord_2_block(r, c);
        let is_candidate = self.candidates[r][c][to_remove];
        self.candidate_cnt[r][c] -= is_candidate as i8;
        self.grid_cnt_for_candidate_in_row[r][to_remove] -= is_candidate as i8;
        self.grid_cnt_for_candidate_in_col[c][to_remove] -= is_candidate as i8;
        self.grid_cnt_for_candidate_in_blk[b][to_remove] -= is_candidate as i8;
        self.candidates[r][c][to_remove] = false;
    }
    fn add_candidate_of_grid(&mut self, r: usize, c: usize, to_add: i8) {
        let to_add = to_add as usize;
        let b = coord_2_block(r, c);
        let is_candidate = self.candidates[r][c][to_add];
        self.candidate_cnt[r][c] += !is_candidate as i8;
        self.grid_cnt_for_candidate_in_row[r][to_add] += !is_candidate as i8;
        self.grid_cnt_for_candidate_in_col[c][to_add] += !is_candidate as i8;
        self.grid_cnt_for_candidate_in_blk[b][to_add] += !is_candidate as i8;
        self.candidates[r][c][to_add] = true;
    }
}

impl Grid for SudokuPuzzleFull {
    fn new(puzzle: [[i8; 9]; 9]) -> Self {
        let mut res = Self {
            board: [[0; 9]; 9],
            candidates: [[[true; 10]; 9]; 9],
            candidate_cnt: [[9; 9]; 9],
            grid_cnt_for_candidate_in_row: [[9; 10]; 9],
            grid_cnt_for_candidate_in_col: [[9; 10]; 9],
            grid_cnt_for_candidate_in_blk: [[9; 10]; 9],
            history: vec![],
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
