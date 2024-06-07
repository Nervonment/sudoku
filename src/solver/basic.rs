use crate::{
    state::{
        CandidatesSettable, Fillable, State, TrackingCandidateCountOfCell, TrackingCandidates,
        TrackingCellCountOfCandidate,
    },
    techniques::{hidden_single_blk, hidden_single_col, hidden_single_row}, Grid,
};

use super::{Grader, Solver};

pub struct BasicSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfCell
        + TrackingCellCountOfCandidate,
{
    puzzle: Grid,
    state: T,
    solution_cnt: u32,
    invoke_cnt: i32,
}

impl<T> BasicSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfCell
        + TrackingCellCountOfCandidate,
{
    fn init_search(&mut self) {
        self.solution_cnt = 0;
        self.invoke_cnt = 0;
        self.state = T::from(self.puzzle);
    }

    fn search(&mut self, solution_cnt_needed: u32) -> bool {
        self.invoke_cnt += 1;
        if self.state.grid().0.iter().flatten().all(|v| *v > 0) {
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
            self.state.fill_cell(r, c, num);
            if self.search(solution_cnt_needed) {
                return true;
            }
            self.state.unfill_cell(r, c);
            return false;
        }

        // 实在不行，找一个候选数字最少的空随便猜一个填上
        let mut min_candidate_cnt = 10;
        let mut grid = (0, 0);
        'outer: for r in 0..9 {
            for c in 0..9 {
                if self.state.is_cell_empty(r, c) {
                    if self.state.candidate_cnt_of_cell(r, c) == 2 {
                        grid = (r, c);
                        break 'outer;
                    }
                    if self.state.candidate_cnt_of_cell(r, c) < min_candidate_cnt {
                        grid = (r, c);
                        min_candidate_cnt = self.state.candidate_cnt_of_cell(r, c);
                    }
                }
            }
        }
        let (r, c) = grid;
        for num in 1..=9 {
            if self.state.is_candidate_of(r, c, num) {
                self.state.fill_cell(r, c, num);
                if self.search(solution_cnt_needed) {
                    return true;
                }
                self.state.unfill_cell(r, c);
            }
        }

        false
    }
}

impl<T> From<Grid> for BasicSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfCell
        + TrackingCellCountOfCandidate,
{
    fn from(puzzle: Grid) -> Self {
        Self {
            puzzle,
            state: T::from(puzzle),
            solution_cnt: 0,
            invoke_cnt: 0,
        }
    }
}

impl<T> Solver for BasicSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfCell
        + TrackingCellCountOfCandidate,
{
    fn any_solution(&mut self) -> Option<Grid> {
        self.init_search();
        if self.search(1) {
            return Some(self.state.grid());
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

impl<T> Grader<i32> for BasicSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfCell
        + TrackingCellCountOfCandidate,
{
    fn difficulty(&self) -> i32 {
        self.invoke_cnt
    }
}
