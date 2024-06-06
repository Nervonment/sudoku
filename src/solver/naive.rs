use crate::{
    state::{
        CandidatesSettable, Fillable, State, TrackingCandidateCountOfGrid, TrackingCandidates,
        TrackingGridCountOfCandidate,
    },
    techniques::{hidden_single_blk, hidden_single_col, hidden_single_row},
};

use super::{Grader, Solver};

pub struct NaiveSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfGrid
        + TrackingGridCountOfCandidate,
{
    puzzle: [[i8; 9]; 9],
    state: T,
    solution_cnt: u32,
    invoke_cnt: i32,
}

impl<T> NaiveSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfGrid
        + TrackingGridCountOfCandidate,
{
    fn init_search(&mut self) {
        self.solution_cnt = 0;
        self.invoke_cnt = 0;
        self.state = T::from(self.puzzle);
    }

    fn search(&mut self, solution_cnt_needed: u32) -> bool {
        self.invoke_cnt += 1;
        if self.state.board().iter().flatten().all(|v| *v > 0) {
            self.solution_cnt += 1;
            return solution_cnt_needed <= self.solution_cnt;
        }

        let step = hidden_single_row(&self.state).unwrap_or(
            hidden_single_col(&self.state)
                .unwrap_or(hidden_single_blk(&self.state).unwrap_or((0, 0, 0))),
        );
        // 如果可以通过 hidden single 确定下一步填的数字
        if step.2 > 0 {
            let (r, c, num) = step;
            self.state.fill_grid(r, c, num);
            if self.search(solution_cnt_needed) {
                return true;
            }
            self.state.unfill_grid(r, c);
            return false;
        }

        false
    }
}

impl<T> From<[[i8; 9]; 9]> for NaiveSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfGrid
        + TrackingGridCountOfCandidate,
{
    fn from(puzzle: [[i8; 9]; 9]) -> Self {
        Self {
            puzzle,
            state: T::from(puzzle),
            solution_cnt: 0,
            invoke_cnt: 0,
        }
    }
}

impl<T> Solver for NaiveSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfGrid
        + TrackingGridCountOfCandidate,
{
    fn any_solution(&mut self) -> Option<[[i8; 9]; 9]> {
        self.init_search();
        if self.search(1) {
            return Some(self.state.board());
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

impl<T> Grader<i32> for NaiveSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfGrid
        + TrackingGridCountOfCandidate,
{
    fn difficulty(&self) -> i32 {
        self.invoke_cnt
    }
}
