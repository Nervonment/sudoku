use crate::{
    state::{State, TrackingCandidates, TrackingCellCountOfCandidate},
    utils::block_idx_2_coord,
};

use super::{House, ReducingCandidates, Technique};

fn hidden_pair<T, F1, F2>(
    state: &T,
    cell_cnt_of_candidate: F1,
    coord_transform: F2,
) -> Option<(
    (usize, usize),
    Vec<i8>,
    (usize, usize),
    Vec<i8>,
    i8,
    i8,
    usize,
)>
where
    T: State + TrackingCandidates,
    F1: Fn(&T, usize, i8) -> i8,
    F2: Fn(usize, usize) -> (usize, usize),
{
    for i in 0..9 {
        let nums: Vec<i8> = (1..=9)
            .filter(|num| cell_cnt_of_candidate(state, i, *num) == 2)
            .collect();
        for i1 in 0..nums.len() {
            for i2 in 0..i1 {
                let num1 = nums[i1];
                let num2 = nums[i2];
                if (0..9).all(|j| {
                    let (r, c) = coord_transform(i, j);
                    !state.is_cell_empty(r, c)
                        || state.is_candidate_of(r, c, num1) == state.is_candidate_of(r, c, num2)
                }) {
                    let mut jiter = (0..9).filter(|j| {
                        let (r, c) = coord_transform(i, *j);
                        state.is_cell_empty(r, c) && state.is_candidate_of(r, c, nums[i1])
                    });
                    let j1 = jiter.next().unwrap();
                    let j2 = jiter.next().unwrap();
                    let (r1, c1) = coord_transform(i, j1);
                    let (r2, c2) = coord_transform(i, j2);
                    let removes_1: Vec<i8> = (1..=9)
                        .filter(|n| *n != num1 && *n != num2 && state.is_candidate_of(r1, c1, *n))
                        .collect();
                    let removes_2: Vec<i8> = (1..=9)
                        .filter(|n| *n != num1 && *n != num2 && state.is_candidate_of(r2, c2, *n))
                        .collect();
                    if !removes_1.is_empty() || !removes_2.is_empty() {
                        return Some((
                            (r1, c1),
                            removes_1,
                            (r2, c2),
                            removes_2,
                            nums[i1],
                            nums[i2],
                            i,
                        ));
                    }
                }
            }
        }
    }
    None
}

#[derive(Clone, Debug)]
pub struct HiddenPairInfo {
    pub house: House,
    pub nums: [i8; 2],
    pub rem_cell_1: (usize, usize),
    pub rem_nums_1: Vec<i8>,
    pub rem_cell_2: (usize, usize),
    pub rem_nums_2: Vec<i8>,
}

pub struct HiddenPairRow(pub Option<HiddenPairInfo>);
impl<T> Technique<T> for HiddenPairRow
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn check(state: &T) -> Self {
        Self(
            hidden_pair(
                state,
                |p, r, num| p.cell_cnt_of_candidate_in_row(r, num),
                |r, c| (r, c),
            )
            .map(|res| HiddenPairInfo {
                house: House::Row(res.6),
                nums: [res.4, res.5],
                rem_cell_1: res.0,
                rem_nums_1: res.1,
                rem_cell_2: res.2,
                rem_nums_2: res.3,
            }),
        )
    }
    fn score(&self) -> f32 {
        3.4
    }
}
impl Into<Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>>> for HiddenPairRow {
    fn into(self) -> Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>> {
        self.0.map(|info| {
            vec![
                (vec![info.rem_cell_1], info.rem_nums_1),
                (vec![info.rem_cell_2], info.rem_nums_2),
            ]
        })
    }
}
impl<T> ReducingCandidates<T> for HiddenPairRow where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate
{
}

pub struct HiddenPairColumn(pub Option<HiddenPairInfo>);
impl<T> Technique<T> for HiddenPairColumn
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn check(state: &T) -> Self {
        Self(
            hidden_pair(
                state,
                |p, c, num| p.cell_cnt_of_candidate_in_col(c, num),
                |c, r| (r, c),
            )
            .map(|res| HiddenPairInfo {
                house: House::Column(res.6),
                nums: [res.4, res.5],
                rem_cell_1: res.0,
                rem_nums_1: res.1,
                rem_cell_2: res.2,
                rem_nums_2: res.3,
            }),
        )
    }
    fn score(&self) -> f32 {
        3.4
    }
}
impl Into<Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>>> for HiddenPairColumn {
    fn into(self) -> Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>> {
        self.0.map(|info| {
            vec![
                (vec![info.rem_cell_1], info.rem_nums_1),
                (vec![info.rem_cell_2], info.rem_nums_2),
            ]
        })
    }
}
impl<T> ReducingCandidates<T> for HiddenPairColumn where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate
{
}
pub struct HiddenPairBlock(pub Option<HiddenPairInfo>);
impl<T> Technique<T> for HiddenPairBlock
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn check(state: &T) -> Self {
        Self(
            hidden_pair(
                state,
                |p, b, num| p.cell_cnt_of_candidate_in_blk(b, num),
                block_idx_2_coord,
            )
            .map(|res| HiddenPairInfo {
                house: House::Block(res.6),
                nums: [res.4, res.5],
                rem_cell_1: res.0,
                rem_nums_1: res.1,
                rem_cell_2: res.2,
                rem_nums_2: res.3,
            }),
        )
    }
    fn score(&self) -> f32 {
        3.4
    }
}
impl Into<Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>>> for HiddenPairBlock {
    fn into(self) -> Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>> {
        self.0.map(|info| {
            vec![
                (vec![info.rem_cell_1], info.rem_nums_1),
                (vec![info.rem_cell_2], info.rem_nums_2),
            ]
        })
    }
}
impl<T> ReducingCandidates<T> for HiddenPairBlock where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate
{
}
