use super::utils::coord_2_block;

use crate::game::utils::next_blank;

use rand::prelude::*;

pub struct SudokuSolver {
    board: [[i8; 9]; 9],    // 棋盘
    row: [[bool; 10]; 9],   // row[r][num] = 第r行是否存在数num
    col: [[bool; 10]; 9],   // 同理
    block: [[bool; 10]; 9], // 同理

    tmp_board: [[i8; 9]; 9],
    solution_cnt: i32, // 解的数量
}

impl SudokuSolver {
    pub fn new(board: [[i8; 9]; 9]) -> Self {
        Self {
            board,
            row: [[false; 10]; 9],
            col: [[false; 10]; 9],
            block: [[false; 10]; 9],
            tmp_board: board,
            solution_cnt: 0,
        }
    }

    // 获取（随机的）一个解
    pub fn get_solution(&mut self) -> Option<[[i8; 9]; 9]> {
        self.init_search();
        if self.search(0, 0, true, true) {
            return Some(self.tmp_board);
        }
        None
    }

    // 获取解的总数
    // 在剩余空格比较多的时候会很慢
    pub fn get_solution_cnt(&mut self) -> i32 {
        self.init_search();
        self.search(0, 0, false, false);
        self.solution_cnt
    }

    // 是否具有唯一解
    pub fn have_unique_solution(&mut self) -> bool {
        self.init_search();
        self.search(0, 0, false, true);
        self.solution_cnt == 1
    }

    fn init_search(&mut self) {
        self.tmp_board = self.board;
        self.row = [[false; 10]; 9];
        self.col = [[false; 10]; 9];
        self.block = [[false; 10]; 9];
        self.solution_cnt = 0;
        for r in 0..9 {
            for c in 0..9 {
                self.row[r][self.board[r][c] as usize] = true;
                self.col[c][self.board[r][c] as usize] = true;
                self.block[coord_2_block(r as i8, c as i8) as usize][self.board[r][c] as usize] =
                    true;
            }
        }
    }

    fn search(
        &mut self,
        mut r: i8,                  // 开始搜索的行
        mut c: i8,                  // 开始搜索的列
        shut_while_found: bool,     // 第一次搜索到解时结束搜索
        shut_while_found_2nd: bool, // 第二次搜索到解时结束搜索（用于判断唯一解）
    ) -> bool {
        // 移动到下一个空格
        let coord = next_blank(r, c, &self.tmp_board);
        if coord == None {
            self.solution_cnt += 1;
            return true;
        }
        (r, c) = coord.unwrap();

        let b = coord_2_block(r, c);

        // 按照随机顺序搜索
        let mut nums: Vec<i8> = (1..10).collect();
        nums.shuffle(&mut rand::thread_rng());
        for num in nums {
            if !self.row[r as usize][num as usize]
                && !self.col[c as usize][num as usize]
                && !self.block[b as usize][num as usize]
            {
                // 尝试
                self.tmp_board[r as usize][c as usize] = num;
                self.row[r as usize][num as usize] = true;
                self.col[c as usize][num as usize] = true;
                self.block[b as usize][num as usize] = true;

                if self.search(r, c, shut_while_found, shut_while_found_2nd) {
                    if shut_while_found || (self.solution_cnt == 2 && shut_while_found_2nd) {
                        return true;
                    }
                }

                // 回溯
                self.tmp_board[r as usize][c as usize] = 0;
                self.row[r as usize][num as usize] = false;
                self.col[c as usize][num as usize] = false;
                self.block[b as usize][num as usize] = false;
            }
        }

        false
    }
}
