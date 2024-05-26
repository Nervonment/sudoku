use std::time::Instant;

use rand::random;

use crate::game::solver::SudokuSolverH;

use super::solver::{SudokuSolver, SudokuSolverS};

// 尝试生成最多有max_blank_cnt个格子的数独谜面
pub fn random_sudoku_puzzle(max_blank_cnt: i32) -> [[i8; 9]; 9] {
    let mut puzzle = SudokuSolverS::new([[0; 9]; 9]).get_solution().unwrap();
    let begin_time = Instant::now();
    for _ in 0..max_blank_cnt {
        loop {
            // 随机选取非空格
            let (mut r, mut c) = (random::<usize>() % 9, random::<usize>() % 9);
            while puzzle[r][c] == 0 {
                (r, c) = (random::<usize>() % 9, random::<usize>() % 9);
            }
            // 挖空，判断是否有唯一解
            let tmp = puzzle[r][c];
            puzzle[r][c] = 0;
            let mut solver = SudokuSolverH::new(puzzle);
            if solver.have_unique_solution() {
                break;
            }
            puzzle[r][c] = tmp;
            // 如果尝试了很多次以后都没有找到新的空可以挖，
            // 就直接返回
            if begin_time.elapsed().as_secs() > 1 {
                return puzzle;
            }
        }
    }

    puzzle
}
