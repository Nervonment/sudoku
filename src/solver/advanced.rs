use crate::{
    state::{
        CandidatesSettable, Fillable, State, TrackingCandidateCountOfCell, TrackingCandidates,
        TrackingCellCountOfCandidate,
    },
    techniques::{
        hidden_subsets::{HiddenPairBlock, HiddenPairColumn, HiddenPairRow},
        locked_candidates::Pointing,
        naked_subsets::{NakedPairBlock, NakedPairColumn, NakedPairRow},
        singles::{HiddenSingleBlock, HiddenSingleColumn, HiddenSingleRow, NakedSingle},
        Direct, ReducingCandidates,
    },
    Grid,
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
    puzzle: Grid,
    state: T,
    solution_cnt: u32,
    tmp_score: f32,
    score: f32,
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
        if self.state.grid().0.iter().flatten().all(|v| *v > 0) {
            self.solution_cnt += 1;
            self.score = self.tmp_score;
            return solution_cnt_needed <= self.solution_cnt;
        }

        let direct_techniques = [
            HiddenSingleBlock::fillable,
            HiddenSingleRow::fillable,
            HiddenSingleColumn::fillable,
            NakedSingle::fillable,
        ];

        for technique in direct_techniques {
            let (fillable, score) = technique(&self.state);
            if fillable.is_some() {
                let (r, c, num) = fillable.unwrap();
                self.state.fill_cell(r, c, num);
                self.tmp_score += score;
                if self.search(solution_cnt_needed) {
                    return true;
                }
                self.state.unfill_cell(r, c);
                self.tmp_score -= score;
                return false;
            }
        }

        let reducing_techniques = [
            Pointing::reducible,
            NakedPairRow::reducible,
            NakedPairColumn::reducible,
            NakedPairBlock::reducible,
            HiddenPairRow::reducible,
            HiddenPairColumn::reducible,
            HiddenPairBlock::reducible,
        ];

        for technique in reducing_techniques {
            let (reducible, score) = technique(&self.state);
            if reducible.is_some() {
                let rems = reducible.unwrap();
                for (cells, nums) in &rems {
                    for (r, c) in cells {
                        for num in nums {
                            self.state.remove_candidate_of_cell(*r, *c, *num);
                        }
                    }
                }
                self.tmp_score += score;
                if self.search(solution_cnt_needed) {
                    return true;
                }
                for (cells, nums) in &rems {
                    for (r, c) in cells {
                        for num in nums {
                            self.state.add_candidate_of_cell(*r, *c, *num);
                        }
                    }
                }
                self.tmp_score -= score;
                return false;
            }
        }

        // TODO: Claiming, Triplet, Fish

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

impl<T> From<Grid> for AdvancedSolver<T>
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
            tmp_score: 0.0,
            score: 0.0,
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
