use std::vec;

use crate::game::utils::{coord_2_block, next_blank};

use rand::prelude::*;

use super::utils::block_idx_2_coord;

pub trait SudokuSolver {
    // 获取任意一个解
    fn get_solution(&mut self) -> Option<[[i8; 9]; 9]>;
    // 获取解的总数
    fn get_solution_cnt(&mut self) -> i32;
    // 判断是否具有唯一解
    fn have_unique_solution(&mut self) -> bool;
}

pub struct SudokuSolverS {
    board: [[i8; 9]; 9],    // 棋盘
    row: [[bool; 10]; 9],   // row[r][num] = 第r行是否存在数num
    col: [[bool; 10]; 9],   // 同理
    block: [[bool; 10]; 9], // 同理

    tmp_board: [[i8; 9]; 9],
    solution_cnt: i32, // 解的数量
}

impl SudokuSolverS {
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

impl SudokuSolver for SudokuSolverS {
    fn get_solution(&mut self) -> Option<[[i8; 9]; 9]> {
        self.init_search();
        if self.search(0, 0, true, true) {
            return Some(self.tmp_board);
        }
        None
    }

    fn get_solution_cnt(&mut self) -> i32 {
        self.init_search();
        self.search(0, 0, false, false);
        self.solution_cnt
    }

    fn have_unique_solution(&mut self) -> bool {
        self.init_search();
        self.search(0, 0, false, true);
        self.solution_cnt == 1
    }
}

pub struct SudokuSolverH {
    board: [[i8; 9]; 9], // 棋盘

    row: [[bool; 10]; 9],   // row[r][num] = 第r行是否存在数num
    col: [[bool; 10]; 9],   // 同理
    block: [[bool; 10]; 9], // 同理
    tmp_board: [[i8; 9]; 9],
    solution_cnt: i32, // 解的数量
    pub invoke_cnt: i32,
    pub unsure_cnt: i32,
}

impl SudokuSolverH {
    pub fn new(board: [[i8; 9]; 9]) -> Self {
        Self {
            board,
            row: [[false; 10]; 9],
            col: [[false; 10]; 9],
            block: [[false; 10]; 9],
            tmp_board: board,
            solution_cnt: 0,
            invoke_cnt: 0,
            unsure_cnt: 0,
        }
    }

    pub fn init_search(&mut self) {
        self.solution_cnt = 0;
        self.tmp_board = self.board;
        self.row = [[false; 10]; 9];
        self.col = [[false; 10]; 9];
        self.block = [[false; 10]; 9];
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
        shut_while_found: bool,     // 第一次搜索到解时结束搜索
        shut_while_found_2nd: bool, // 第二次搜索到解时结束搜索（用于判断唯一解）
    ) -> bool {
        self.invoke_cnt += 1;
        let next_steps = self.next_steps();
        if next_steps == None {
            // 当前无法继续填充，且棋盘未满，则抵达非解叶节点
            for r in 0..9 {
                for c in 0..9 {
                    if self.tmp_board[r][c] == 0 {
                        return false;
                    }
                }
            }
            // 棋盘已满，找到完全解
            self.solution_cnt += 1;
            return true;
        }
        let next_steps = next_steps.unwrap();
        if next_steps.len() > 1 {
            self.unsure_cnt += 1;
        }

        for (r, c, num) in next_steps {
            let b = coord_2_block(r, c);
            // 尝试
            self.tmp_board[r as usize][c as usize] = num;
            self.row[r as usize][num as usize] = true;
            self.col[c as usize][num as usize] = true;
            self.block[b as usize][num as usize] = true;

            if self.search(shut_while_found, shut_while_found_2nd) {
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

        false
    }

    // 获取下一步行动的集合，其中元素为 (r, c, num)
    // 如果再填一步就会违反约束，或者棋盘满了，返回 None
    fn next_steps(&self) -> Option<Vec<(i8, i8, i8)>> {
        // 令 A_R(r, num) 为第 r 行中数字 num 可能填的位置的集合
        // 令 A_C(c, num) 为第 c 列中数字 num 可能填的位置的集合
        // 令 A_B(b, num) 为第 b 宫中数字 num 可能填的位置的集合
        // A_R, A_C, A_B 中的元素为可能填的位置 (r, c) 和填的数字 num 组成的三元组 (r, c, num)
        // 则 least_viable_options 为所有非空的 A_R, A_C, A_B 中元素个数最小者
        let mut least_viable_options = vec![]; // Vec<(r, c, num)>
        let mut least_viable_options_cnt = 10; // least_viable_options.len()

        // 是否存在上述非空的 A_R, A_C, A_B
        // 如果未 hit 且棋盘未满，则当前在任意格填任意数字都将违反数独约束条件，无法继续填充
        let mut hit = false;

        // 检查和更新 least_viable_options
        let mut check_least = |cur_viable_options: Vec<(i8, i8, i8)>| {
            if cur_viable_options.len() > 0 && cur_viable_options.len() < least_viable_options_cnt {
                least_viable_options_cnt = cur_viable_options.len();
                least_viable_options = cur_viable_options;
            }
        };

        // 检查所有 A_R
        for r in 0..9 {
            for num in 1..10 {
                if !self.row[r][num] {
                    let mut cur_viable_options = vec![]; // A_R(r, num)
                    for c in 0..9 {
                        if self.tmp_board[r][c] == 0
                            && !self.col[c][num]
                            && !self.block[coord_2_block(r as i8, c as i8) as usize][num]
                        {
                            hit = true;
                            cur_viable_options.push((r as i8, c as i8, num as i8));
                        }
                    }
                    // 如果检查到能够确定的格子，直接返回
                    if cur_viable_options.len() == 1 {
                        return Some(cur_viable_options);
                    }
                    check_least(cur_viable_options);
                }
            }
        }
        // 检查所有 A_C
        for c in 0..9 {
            for num in 1..10 {
                if !self.col[c][num] {
                    let mut cur_viable_options = vec![]; // A_C(c, num)
                    for r in 0..9 {
                        if self.tmp_board[r][c] == 0
                            && !self.row[r][num]
                            && !self.block[coord_2_block(r as i8, c as i8) as usize][num]
                        {
                            hit = true;
                            cur_viable_options.push((r as i8, c as i8, num as i8));
                        }
                    }
                    // 如果检查到能够确定的格子，直接返回
                    if cur_viable_options.len() == 1 {
                        return Some(cur_viable_options);
                    }
                    check_least(cur_viable_options);
                }
            }
        }
        // 检查所有 A_B
        for b in 0..9 {
            for num in 1..10 {
                if !self.block[b][num] {
                    let mut cur_viable_options = vec![]; // A_B(b, num)
                    for idx_in_b in 0..9 {
                        let (r, c) = block_idx_2_coord(b, idx_in_b);
                        if self.tmp_board[r][c] == 0 && !self.row[r][num] && !self.col[c][num] {
                            hit = true;
                            cur_viable_options.push((r as i8, c as i8, num as i8));
                        }
                    }
                    // 如果检查到能够确定的格子，直接返回
                    if cur_viable_options.len() == 1 {
                        return Some(cur_viable_options);
                    }
                    check_least(cur_viable_options);
                }
            }
        }
        if !hit {
            return None;
        }
        Some(least_viable_options)
    }

    pub fn get_next_steps(&mut self) -> Option<Vec<(i8, i8, i8)>> {
        self.init_search();
        self.next_steps()
    }
}

impl SudokuSolver for SudokuSolverH {
    fn get_solution(&mut self) -> Option<[[i8; 9]; 9]> {
        self.init_search();
        if self.search(true, true) {
            return Some(self.tmp_board);
        }
        None
    }

    fn get_solution_cnt(&mut self) -> i32 {
        self.init_search();
        self.search(false, false);
        self.solution_cnt
    }

    fn have_unique_solution(&mut self) -> bool {
        self.init_search();
        self.search(false, true);
        self.solution_cnt == 1
    }
}
