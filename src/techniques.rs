pub trait Technique {
    fn check(state: &FullState) -> Self;
    // TODO
    // fn decription() -> String;
}

pub trait Direct: Technique {
    fn fillable(&self) -> Option<(usize, usize, i8)>;
}

pub trait ReducingCandidates: Technique {
    // 如果返回值为 Some(removes)，
    // 对于 removes 中的任意元素 (cells, nums)，
    // cells 与 nums 中元素的笛卡尔积为所有的移除对，
    // 即：可以从 cells 中的任意格的候选数中移除 nums 中的任意数
    fn reducible(&self) -> Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>>;
}

#[derive(Clone, Copy)]
pub enum House {
    Row(usize),
    Column(usize),
    Block(usize),
}

pub mod hidden_subsets;
pub mod locked_candidates;
pub mod naked_subsets;
pub mod singles;

use crate::state::full_state::FullState;

use super::{
    state::{
        State, TrackingCandidateCountOfCell, TrackingCandidates, TrackingCellCountOfCandidate,
    },
    utils::{block_idx_2_coord, coord_2_block},
};

// TODO: Add the Direct variants for hidden_pair and pointing


fn naked_pair<T, F>(
    state: &T,
    coord_transform: F,
) -> Option<(
    (usize, usize),
    (usize, usize),
    i8,
    Vec<(usize, usize)>,
    i8,
    Vec<(usize, usize)>,
)>
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
    F: Fn(usize, usize) -> (usize, usize),
{
    for i in 0..9 {
        let js: Vec<usize> = (0..9)
            .filter(|j| {
                let (r, c) = coord_transform(i, *j);
                state.is_cell_empty(r, c) && state.candidate_cnt_of_cell(r, c) == 2
            })
            .collect();
        for i1 in 0..js.len() {
            for i2 in 0..i1 {
                let j1 = js[i1];
                let j2 = js[i2];
                let (r1, c1) = coord_transform(i, j1);
                let (r2, c2) = coord_transform(i, j2);
                if (1..=9).all(|num| {
                    state.is_candidate_of(r1, c1, num) == state.is_candidate_of(r2, c2, num)
                }) {
                    let mut num_iter = (1..=9).filter(|num| state.is_candidate_of(r1, c1, *num));
                    let num1 = num_iter.next().unwrap();
                    let num2 = num_iter.next().unwrap();
                    let removes_1: Vec<(usize, usize)> = (0..9)
                        .filter(|j| {
                            let (r, c) = coord_transform(i, *j);
                            *j != j1
                                && *j != j2
                                && state.is_cell_empty(r, c)
                                && state.is_candidate_of(r, c, num1)
                        })
                        .map(|j| coord_transform(i, j))
                        .collect();
                    let removes_2: Vec<(usize, usize)> = (0..9)
                        .filter(|j| {
                            let (r, c) = coord_transform(i, *j);
                            *j != j1
                                && *j != j2
                                && state.is_cell_empty(r, c)
                                && state.is_candidate_of(r, c, num2)
                        })
                        .map(|j| coord_transform(i, j))
                        .collect();
                    if !removes_1.is_empty() || !removes_2.is_empty() {
                        return Some(((r1, c1), (r2, c2), num1, removes_1, num2, removes_2));
                    }
                }
            }
        }
    }
    None
}

pub fn naked_pair_row(
    state: &(impl State + TrackingCandidates + TrackingCandidateCountOfCell),
) -> Option<(
    (usize, usize),
    (usize, usize),
    i8,
    Vec<(usize, usize)>,
    i8,
    Vec<(usize, usize)>,
)> {
    naked_pair(state, |r, c| (r, c))
}

pub fn naked_pair_col(
    state: &(impl State + TrackingCandidates + TrackingCandidateCountOfCell),
) -> Option<(
    (usize, usize),
    (usize, usize),
    i8,
    Vec<(usize, usize)>,
    i8,
    Vec<(usize, usize)>,
)> {
    naked_pair(state, |c, r| (r, c))
}

pub fn naked_pair_blk(
    state: &(impl State + TrackingCandidates + TrackingCandidateCountOfCell),
) -> Option<(
    (usize, usize),
    (usize, usize),
    i8,
    Vec<(usize, usize)>,
    i8,
    Vec<(usize, usize)>,
)> {
    naked_pair(state, block_idx_2_coord)
}
