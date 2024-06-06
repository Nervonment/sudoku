use super::{next_blank, Solver};

use crate::state::{Fillable, State, TrackingCandidates};

use rand::prelude::*;

pub struct StochasticSolver<T>
where
    T: State + Fillable + TrackingCandidates,
{
    puzzle: [[i8; 9]; 9],
    state: T,
    solution_cnt: u32,
}

impl<T> StochasticSolver<T>
where
    T: State + Fillable + TrackingCandidates,
{
    fn init_search(&mut self) {
        self.solution_cnt = 0;
        self.state = T::from(self.puzzle);
    }

    fn search(&mut self, r: usize, c: usize, solution_cnt_needed: u32) -> bool {
        let coord = next_blank(r, c, &self.state);
        if coord.is_none() {
            self.solution_cnt += 1;
            return true;
        }
        let (r, c) = coord.unwrap();

        let mut nums: Vec<i8> = (1..=9).collect();
        nums.shuffle(&mut rand::thread_rng());
        for num in nums {
            if self.state.is_candidate_of(r, c, num) {
                self.state.fill_cell(r, c, num);

                if self.search(r, c, solution_cnt_needed)
                    && solution_cnt_needed <= self.solution_cnt
                {
                    return true;
                }

                self.state.unfill_cell(r, c);
            }
        }

        false
    }
}

impl<T> From<[[i8; 9]; 9]> for StochasticSolver<T>
where
    T: State + Fillable + TrackingCandidates,
{
    fn from(puzzle: [[i8; 9]; 9]) -> Self {
        Self {
            puzzle,
            state: T::from(puzzle),
            solution_cnt: 0,
        }
    }
}

impl<T> Solver for StochasticSolver<T>
where
    T: State + Fillable + TrackingCandidates,
{
    fn any_solution(&mut self) -> Option<[[i8; 9]; 9]> {
        self.init_search();
        if self.search(0, 0, 1) {
            return Some(self.state.grid());
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
