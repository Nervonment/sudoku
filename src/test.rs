use rand::random;

use crate::{
    generator::random_sudoku_puzzle,
    judge::judge_sudoku,
    solver::{advanced::AdvancedSolver, stochastic::StochasticSolver, Solver},
    state::{
        full_state::FullState, simple_state::SimpleState, CandidatesSettable, Fillable, State,
        TrackingCandidateCountOfGrid, TrackingCandidates, TrackingGridCountOfCandidate,
    },
    techniques::{
        hidden_pair_row, hidden_single_blk, hidden_single_col, hidden_single_row, naked_single,
    },
    utils::{block_idx_2_coord, coord_2_block_idx},
};

fn random_sudoku_puzzle_normal() -> [[i8; 9]; 9] {
    random_sudoku_puzzle::<StochasticSolver<SimpleState>, AdvancedSolver<FullState>, f32>(
        45, 0.0, 1000.0,
    )
}

#[test]
fn sudoku_puzzle() {
    for _ in 0..100 {
        let puzzle = random_sudoku_puzzle_normal();
        let mut puzzle = FullState::from(puzzle);

        let mut moves = vec![];
        for _ in 0..10 {
            let (mut r, mut c) = (random::<usize>() % 9, random::<usize>() % 9);
            while !puzzle.is_grid_empty(r, c) {
                (r, c) = (random::<usize>() % 9, random::<usize>() % 9);
            }
            let num = (random::<u8>() % 9 + 1) as i8;
            moves.push((r, c));
            puzzle.fill_grid(r, c, num);
        }
        while !moves.is_empty() {
            let (r, c) = moves.pop().unwrap();
            puzzle.unfill_grid(r, c);
        }

        for r in 0..9 {
            for c in 0..9 {
                let (b, bidx) = coord_2_block_idx(r, c);
                // 如果格 (r,c) 已经填上数 num，那么同行、同列和同宫的空格子的候选数都不包含 num
                let num = puzzle.grid_val(r, c);
                if num > 0 {
                    for i in 0..9 {
                        if i != c && puzzle.grid_val(r, i) == 0 {
                            assert!(!puzzle.is_candidate_of(r, i, num));
                        }
                        if i != r && puzzle.grid_val(i, c) == 0 {
                            assert!(!puzzle.is_candidate_of(i, c, num));
                        }
                        if i != bidx {
                            let (r1, c1) = block_idx_2_coord(b, bidx);
                            if puzzle.grid_val(r1, c1) == 0 {
                                assert!(!puzzle.is_candidate_of(r1, c1, num));
                            }
                        }
                    }
                }
                // 如果格 (r,c) 未填数，那么他的候选数数量等于候选数数量
                else {
                    let mut candidate_cnt = 0;
                    for num in 1..=9 {
                        candidate_cnt += puzzle.is_candidate_of(r, c, num) as i8;
                    }
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_grid_in_row(r, c));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_grid_in_col(c, r));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_grid_in_blk(b, bidx));
                }
            }
        }

        for _ in 0..10 {
            let (mut r, mut c) = (random::<usize>() % 9, random::<usize>() % 9);
            while !puzzle.is_grid_empty(r, c) {
                (r, c) = (random::<usize>() % 9, random::<usize>() % 9);
            }
            let num = (random::<u8>() % 9 + 1) as i8;
            puzzle.remove_candidate_of_grid(r, c, num);
        }

        for r in 0..9 {
            for c in 0..9 {
                let (b, bidx) = coord_2_block_idx(r, c);
                if puzzle.is_grid_empty(r, c) {
                    let mut candidate_cnt = 0;
                    for num in 1..=9 {
                        candidate_cnt += puzzle.is_candidate_of(r, c, num) as i8;
                    }
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_grid_in_row(r, c));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_grid_in_col(c, r));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_grid_in_blk(b, bidx));
                }
            }
        }

        for num in 1..=9 {
            for r in 0..9 {
                // grid_cnt 是第 r 行中候选数列表包含 num 的空格子数
                let mut grid_cnt = 0;
                for c in 0..9 {
                    grid_cnt +=
                        (puzzle.is_candidate_of(r, c, num) && puzzle.grid_val(r, c) == 0) as i8;
                }
                assert_eq!(grid_cnt, puzzle.grid_cnt_of_candidate_in_row(r, num));
            }
            for c in 0..9 {
                // grid_cnt 是第 c 列中候选数列表包含 num 的空格子数
                let mut grid_cnt = 0;
                for r in 0..9 {
                    grid_cnt +=
                        (puzzle.is_candidate_of(r, c, num) && puzzle.grid_val(r, c) == 0) as i8;
                }
                assert_eq!(grid_cnt, puzzle.grid_cnt_of_candidate_in_col(c, num));
            }
            for b in 0..9 {
                // grid_cnt 是第 b 宫中候选数列表包含 num 的空格子数
                let mut grid_cnt = 0;
                for bidx in 0..9 {
                    let (r, c) = block_idx_2_coord(b, bidx);
                    grid_cnt +=
                        (puzzle.is_candidate_of(r, c, num) && puzzle.grid_val(r, c) == 0) as i8;
                }
                assert_eq!(grid_cnt, puzzle.grid_cnt_of_candidate_in_blk(b, num));
            }
        }
    }
}

#[test]
fn techniques_single() {
    for _ in 0..100 {
        let puzzle = random_sudoku_puzzle_normal();
        let mut puzzle = FullState::from(puzzle);
        let res_hidden_single_row = hidden_single_row(&puzzle);
        let res_hidden_single_col = hidden_single_col(&puzzle);
        let res_hidden_single_blk = hidden_single_blk(&puzzle);
        let res_naked_single = naked_single(&puzzle);
        let singles = [
            res_hidden_single_row,
            res_hidden_single_col,
            res_hidden_single_blk,
            res_naked_single,
        ];
        for single in singles {
            if single.is_some() {
                let (r, c, num) = single.unwrap();
                puzzle.fill_grid(r, c, num);
                assert!(judge_sudoku(&puzzle.board()).0);
                puzzle.unfill_grid(r, c);
            }
        }
    }
}

#[test]
fn techniques_pair() {
    for _ in 0..10 {
        let mut res_hidden_pair_row = None;
        let mut puzzle = random_sudoku_puzzle_normal();
        while res_hidden_pair_row.is_none() {
            puzzle = random_sudoku_puzzle_normal();
            let puzzle = FullState::from(puzzle);
            res_hidden_pair_row = hidden_pair_row(&puzzle);
        }
        let mut puzzle = FullState::from(puzzle);
        let ((r1, c1), _, (r2, c2), _, num1, num2) = res_hidden_pair_row.clone().unwrap();
        let nums: Vec<i8> = (1..=9).filter(|n| *n != num1 && *n != num2).collect();
        for num in &nums {
            puzzle.remove_candidate_of_grid(r1, c1, *num);
            puzzle.remove_candidate_of_grid(r2, c2, *num);
        }
        let res_hidden_pair_row_1 = hidden_pair_row(&puzzle);
        assert_ne!(res_hidden_pair_row, res_hidden_pair_row_1);
    }
}

#[test]
fn sudoku_solver() {
    for _ in 0..100 {
        let puzzle = random_sudoku_puzzle_normal();
        let mut solver1 = StochasticSolver::<SimpleState>::from(puzzle);
        let mut solver2 = AdvancedSolver::<FullState>::from(puzzle);
        assert!(solver1.have_unique_solution());
        assert!(solver2.have_unique_solution());
        let solution1 = solver1.any_solution().unwrap();
        let solution2 = solver2.any_solution().unwrap();
        assert!(judge_sudoku(&solution1).1);
        assert!(judge_sudoku(&solution2).1);
    }
}
