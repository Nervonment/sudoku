use super::puzzle::{Fillable, Grid, TrackingCandidates};
use rand::prelude::*;

pub trait Solver {
    fn new(puzzle: [[i8; 9]; 9]) -> Self;
    fn any_solution(&mut self) -> Option<[[i8; 9]; 9]>;
    fn solution_cnt(&mut self) -> u32;
    fn have_unique_solution(&mut self) -> bool;
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

    fn next_blank(&self, mut row: usize, mut col: usize) -> Option<(usize, usize)> {
        while row < 9 && !self.puzzle.is_grid_empty(row, col) {
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

    fn search(&mut self, r: usize, c: usize, solution_cnt_needed: u32) -> bool {
        let coord = self.next_blank(r, c);
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
