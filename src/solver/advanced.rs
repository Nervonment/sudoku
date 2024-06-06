use crate::{
    state::{
        CandidatesSettable, Fillable, State, TrackingCandidateCountOfCell, TrackingCandidates,
        TrackingCellCountOfCandidate,
    },
    techniques::{
        hidden_pair_blk, hidden_pair_col, hidden_pair_row, hidden_single_blk, hidden_single_col,
        hidden_single_row, naked_pair_blk, naked_pair_col, naked_pair_row, naked_single, pointing,
    },
};

use super::{Grader, Solver};

pub struct AdvancedSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfCell
        + TrackingCellCountOfCandidate,
{
    puzzle: [[i8; 9]; 9],
    state: T,
    solution_cnt: u32,
    tmp_score: f32,
    score: f32
}

impl<T> AdvancedSolver<T>
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
        self.state = T::from(self.puzzle);
    }

    fn search(&mut self, solution_cnt_needed: u32) -> bool {
        if self.state.grid().iter().flatten().all(|v| *v > 0) {
            self.solution_cnt += 1;
            self.score = self.tmp_score;
            return solution_cnt_needed <= self.solution_cnt;
        }

        let step = hidden_single_row(&self.state).unwrap_or(
            hidden_single_col(&self.state).unwrap_or(
                hidden_single_blk(&self.state)
                    .unwrap_or(naked_single(&self.state).unwrap_or((0, 0, 0))),
            ),
        );
        // 如果可以通过 hidden single 或 naked single 确定下一步填的数字
        if step.2 > 0 {
            let (r, c, num) = step;
            self.state.fill_cell(r, c, num);
            self.tmp_score += 1.5;
            if self.search(solution_cnt_needed) {
                return true;
            }
            self.state.unfill_cell(r, c);
            self.tmp_score -= 1.5;
            return false;
        }

        let ((r1, c1), rem1, (r2, c2), rem2, num1, _) =
            hidden_pair_row(&self.state).unwrap_or(hidden_pair_col(&self.state).unwrap_or(
                hidden_pair_blk(&self.state).unwrap_or(((0, 0), vec![], (0, 0), vec![], 0, 0)),
            ));
        // 如果可以通过 hidden pair 删除一些候选数字
        if num1 > 0 {
            for num in &rem1 {
                self.state.remove_candidate_of_cell(r1, c1, *num);
            }
            for num in &rem2 {
                self.state.remove_candidate_of_cell(r2, c2, *num);
            }
            self.tmp_score += 2.7;
            if self.search(solution_cnt_needed) {
                return true;
            }
            for num in &rem1 {
                self.state.add_candidate_of_cell(r1, c1, *num);
            }
            for num in &rem2 {
                self.state.add_candidate_of_cell(r2, c2, *num);
            }
            self.tmp_score -= 2.7;
            return false;
        }

        let ((_, _), (_, _), num1, rem1, num2, rem2) =
            naked_pair_row(&self.state).unwrap_or(naked_pair_col(&self.state).unwrap_or(
                naked_pair_blk(&self.state).unwrap_or(((0, 0), (0, 0), 0, vec![], 0, vec![])),
            ));
        // 如果可以通过 naked pair 删除一些候选数字
        if num1 > 0 {
            for (r, c) in &rem1 {
                self.state.remove_candidate_of_cell(*r, *c, num1);
            }
            for (r, c) in &rem2 {
                self.state.remove_candidate_of_cell(*r, *c, num2);
            }
            self.tmp_score += 3.0;
            if self.search(solution_cnt_needed) {
                return true;
            }
            for (r, c) in &rem1 {
                self.state.add_candidate_of_cell(*r, *c, num1);
            }
            for (r, c) in &rem2 {
                self.state.add_candidate_of_cell(*r, *c, num2);
            }
            self.tmp_score -= 3.0;
            return false;
        }

        let res_pointing = pointing(&self.state);
        // 如果可以通过 pointing 删除一些候选数字
        if res_pointing.is_some() {
            let (_, num, rems) = res_pointing.unwrap();
            for (r, c) in &rems {
                self.state.remove_candidate_of_cell(*r, *c, num);
            }
            self.tmp_score += 2.2;
            if self.search(solution_cnt_needed) {
                return true;
            }
            for (r, c) in &rems {
                self.state.add_candidate_of_cell(*r, *c, num);
            }
            self.tmp_score -= 2.2;
            return false;
        }

        // TODO: Claiming,  Triplet, X-Wing, Swordfish, XY-Wing, XYZ-Wing

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
                self.tmp_score += 8.0;
                if self.search(solution_cnt_needed) {
                    return true;
                }
                self.tmp_score -= 8.0;
                self.state.unfill_cell(r, c);
            }
        }

        false
    }
}

impl<T> From<[[i8; 9]; 9]> for AdvancedSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfCell
        + TrackingCellCountOfCandidate,
{
    fn from(puzzle: [[i8; 9]; 9]) -> Self {
        Self {
            puzzle,
            state: T::from(puzzle),
            solution_cnt: 0,
            tmp_score: 0.0,
            score: 0.0
        }
    }
}

impl<T> Solver for AdvancedSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfCell
        + TrackingCellCountOfCandidate,
{
    fn any_solution(&mut self) -> Option<[[i8; 9]; 9]> {
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

impl<T> Grader<f32> for AdvancedSolver<T>
where
    T: State
        + Fillable
        + CandidatesSettable
        + TrackingCandidates
        + TrackingCandidateCountOfCell
        + TrackingCellCountOfCandidate,
{
    fn difficulty(&self) -> f32 {
        self.score
    }
}
