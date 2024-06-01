use std::vec;

use super::{
    puzzle::{
        CandidatesSettable, Fillable, Grid, TrackingCandidateCountOfGrid, TrackingCandidates,
        TrackingGridCountOfCandidate,
    },
    techniques::{
        hidden_pair_blk, hidden_pair_col, hidden_pair_row, hidden_single_blk, hidden_single_col,
        hidden_single_row, naked_single,
    },
};
use rand::prelude::*;

pub trait Solver {
    fn new(puzzle: [[i8; 9]; 9]) -> Self;
    fn any_solution(&mut self) -> Option<[[i8; 9]; 9]>;
    fn solution_cnt(&mut self) -> u32;
    fn have_unique_solution(&mut self) -> bool;
}

fn next_blank(mut row: usize, mut col: usize, puzzle: &impl Grid) -> Option<(usize, usize)> {
    while row < 9 && !puzzle.is_grid_empty(row, col) {
        if col == 8 {
            col = 0;
            row += 1;
        } else {
            col += 1
        }
    }
    if row == 9 {
        return None;
    }
    Some((row, col))
}

pub struct StochasticSolver<T>
where
    T: Grid + Fillable + TrackingCandidates,
{
    puzzle_arr: [[i8; 9]; 9],
    puzzle: T,
    solution_cnt: u32,
}

impl<T> StochasticSolver<T>
where
    T: Grid + Fillable + TrackingCandidates,
{
    fn init_search(&mut self) {
        self.solution_cnt = 0;
        self.puzzle = T::new(self.puzzle_arr);
    }

    fn search(&mut self, r: usize, c: usize, solution_cnt_needed: u32) -> bool {
        let coord = next_blank(r, c, &self.puzzle);
        if coord.is_none() {
            self.solution_cnt += 1;
            return true;
        }
        let (r, c) = coord.unwrap();

        let mut nums: Vec<i8> = (1..=9).collect();
        nums.shuffle(&mut rand::thread_rng());
        for num in nums {
            if self.puzzle.is_candidate_of(r, c, num) {
                self.puzzle.fill_grid(r, c, num);

                if self.search(r, c, solution_cnt_needed)
                    && solution_cnt_needed <= self.solution_cnt
                {
                    return true;
                }

                self.puzzle.unfill_grid(r, c);
            }
        }

        false
    }
}

impl<T> Solver for StochasticSolver<T>
where
    T: Grid + Fillable + TrackingCandidates,
{
    fn new(puzzle: [[i8; 9]; 9]) -> Self {
        Self {
            puzzle_arr: puzzle,
            puzzle: T::new(puzzle),
            solution_cnt: 0,
        }
    }

    fn any_solution(&mut self) -> Option<[[i8; 9]; 9]> {
        self.init_search();
        if self.search(0, 0, 1) {
            return Some(self.puzzle.board());
        }
        None
    }

    fn solution_cnt(&mut self) -> u32 {
        self.init_search();
        self.search(0, 0, u32::MAX);
        self.solution_cnt
    }

    fn have_unique_solution(&mut self) -> bool {
        self.init_search();
        self.search(0, 0, 2);
        self.solution_cnt == 1
    }
}

pub struct TechniquesSolver<T>
where
    T: Grid
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfGrid
        + TrackingGridCountOfCandidate,
{
    puzzle_arr: [[i8; 9]; 9],
    puzzle: T,
    solution_cnt: u32,
}

impl<T> TechniquesSolver<T>
where
    T: Grid
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfGrid
        + TrackingGridCountOfCandidate,
{
    fn init_search(&mut self) {
        self.solution_cnt = 0;
        self.puzzle = T::new(self.puzzle_arr);
    }

    fn search(&mut self, solution_cnt_needed: u32) -> bool {
        if self.puzzle.board().iter().flatten().all(|v| *v > 0) {
            self.solution_cnt += 1;
            return solution_cnt_needed <= self.solution_cnt;
        }

        let step = hidden_single_row(&self.puzzle).unwrap_or(
            hidden_single_col(&self.puzzle).unwrap_or(
                hidden_single_blk(&self.puzzle)
                    .unwrap_or(naked_single(&self.puzzle).unwrap_or((0, 0, 0))),
            ),
        );
        // 可以通过 hidden single 或 naked single 确定下一步填的数字
        if step.2 > 0 {
            let (r, c, num) = step;
            self.puzzle.fill_grid(r, c, num);
            if self.search(solution_cnt_needed) {
                return true;
            }
            self.puzzle.unfill_grid(r, c);
            return false;
        }

        let ((r1, c1), rem1, (r2, c2), rem2, num1, _) =
            hidden_pair_row(&self.puzzle).unwrap_or(hidden_pair_col(&self.puzzle).unwrap_or(
                hidden_pair_blk(&self.puzzle).unwrap_or(((0, 0), vec![], (0, 0), vec![], 0, 0)),
            ));
        // 可以通过 hidden pair 删除一些候选数字
        if num1 > 0 {
            for num in &rem1 {
                self.puzzle.remove_candidate_of_grid(r1, c1, *num);
            }
            for num in &rem2 {
                self.puzzle.remove_candidate_of_grid(r2, c2, *num);
            }
            if self.search(solution_cnt_needed) {
                return true;
            }
            for num in &rem1 {
                self.puzzle.add_candidate_of_grid(r1, c1, *num);
            }
            for num in &rem2 {
                self.puzzle.add_candidate_of_grid(r2, c2, *num);
            }
            return false;
        }

        // 实在不行，找一个候选数字最少的空随便猜一个填上
        let mut min_candidate_cnt = 10;
        let mut grid = (0, 0);
        'outer: for r in 0..9 {
            for c in 0..9 {
                if self.puzzle.is_grid_empty(r, c) {
                    if self.puzzle.candidate_cnt_of_grid(r, c) == 2 {
                        grid = (r, c);
                        break 'outer;
                    }
                    if self.puzzle.candidate_cnt_of_grid(r, c) < min_candidate_cnt {
                        grid = (r, c);
                        min_candidate_cnt = self.puzzle.candidate_cnt_of_grid(r, c);
                    }
                }
            }
        }
        let (r, c) = grid;
        for num in 1..=9 {
            if self.puzzle.is_candidate_of(r, c, num) {
                self.puzzle.fill_grid(r, c, num);
                if self.search(solution_cnt_needed) {
                    return true;
                }
                self.puzzle.unfill_grid(r, c);
            }
        }

        false
    }
}

impl<T> Solver for TechniquesSolver<T>
where
    T: Grid
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfGrid
        + TrackingGridCountOfCandidate,
{
    fn new(puzzle: [[i8; 9]; 9]) -> Self {
        Self {
            puzzle_arr: puzzle,
            puzzle: T::new(puzzle),
            solution_cnt: 0,
        }
    }

    fn any_solution(&mut self) -> Option<[[i8; 9]; 9]> {
        self.init_search();
        if self.search(1) {
            return Some(self.puzzle.board());
        }
        None
    }

    fn solution_cnt(&mut self) -> u32 {
        self.init_search();
        self.search(u32::MAX);
        self.solution_cnt
    }

    fn have_unique_solution(&mut self) -> bool {
        self.init_search();
        self.search(2);
        self.solution_cnt == 1
    }
}
