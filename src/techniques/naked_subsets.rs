use crate::{
    state::{State, TrackingCandidateCountOfCell, TrackingCandidates},
    utils::block_idx_2_coord,
};

use super::{House, ReducingCandidates, ReducingCandidatesOption, Technique};

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
    usize,
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
            .collect(); // 筛选出此单元中候选数只有两个的未填格子
        for i1 in 0..js.len() {
            for i2 in 0..i1 {
                let j1 = js[i1];
                let j2 = js[i2];
                let (r1, c1) = coord_transform(i, j1);
                let (r2, c2) = coord_transform(i, j2);
                if (1..=9).all(|num| {
                    state.is_candidate_of(r1, c1, num) == state.is_candidate_of(r2, c2, num)
                }) {
                    // 如果两个格子候选数相同
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
                        return Some(((r1, c1), (r2, c2), num1, removes_1, num2, removes_2, i));
                    }
                }
            }
        }
    }
    None
}

#[derive(Clone, Debug)]
pub struct NakedPairInfo {
    pub house: House,
    pub cells: [(usize, usize); 2],
    pub rem_cells_1: Vec<(usize, usize)>,
    pub rem_num_1: i8,
    pub rem_cells_2: Vec<(usize, usize)>,
    pub rem_num_2: i8,
}

#[derive(Default)]
pub struct NakedPairRow(pub Option<NakedPairInfo>);
impl<T> Technique<T> for NakedPairRow
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
{
    fn analyze(&mut self, state: &T) {
        self.0 = naked_pair(state, |r, c| (r, c)).map(|res| NakedPairInfo {
            house: House::Row(res.6),
            cells: [res.0, res.1],
            rem_cells_1: res.3,
            rem_num_1: res.2,
            rem_cells_2: res.5,
            rem_num_2: res.4,
        })
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        if self.0.is_some() {
            return Some(3.0);
        }
        None
    }
}
impl<T> ReducingCandidates<T> for NakedPairRow
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
{
    fn option(&self) -> Option<ReducingCandidatesOption> {
        self.0.clone().map(|info| {
            ReducingCandidatesOption(vec![
                (info.rem_cells_1, vec![info.rem_num_1]),
                (info.rem_cells_2, vec![info.rem_num_2]),
            ])
        })
    }
}

#[derive(Default)]
pub struct NakedPairColumn(pub Option<NakedPairInfo>);
impl<T> Technique<T> for NakedPairColumn
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
{
    fn analyze(&mut self, state: &T) {
        self.0 = naked_pair(state, |c, r| (r, c)).map(|res| NakedPairInfo {
            house: House::Column(res.6),
            cells: [res.0, res.1],
            rem_cells_1: res.3,
            rem_num_1: res.2,
            rem_cells_2: res.5,
            rem_num_2: res.4,
        })
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        if self.0.is_some() {
            return Some(3.0);
        }
        None
    }
}
impl<T> ReducingCandidates<T> for NakedPairColumn
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
{
    fn option(&self) -> Option<ReducingCandidatesOption> {
        self.0.clone().map(|info| {
            ReducingCandidatesOption(vec![
                (info.rem_cells_1, vec![info.rem_num_1]),
                (info.rem_cells_2, vec![info.rem_num_2]),
            ])
        })
    }
}

#[derive(Default)]
pub struct NakedPairBlock(pub Option<NakedPairInfo>);
impl<T> Technique<T> for NakedPairBlock
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
{
    fn analyze(&mut self, state: &T) {
        self.0 = naked_pair(state, block_idx_2_coord).map(|res| NakedPairInfo {
            house: House::Block(res.6),
            cells: [res.0, res.1],
            rem_cells_1: res.3,
            rem_num_1: res.2,
            rem_cells_2: res.5,
            rem_num_2: res.4,
        })
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        if self.0.is_some() {
            return Some(3.0);
        }
        None
    }
}
impl<T> ReducingCandidates<T> for NakedPairBlock
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
{
    fn option(&self) -> Option<ReducingCandidatesOption> {
        self.0.clone().map(|info| {
            ReducingCandidatesOption(vec![
                (info.rem_cells_1, vec![info.rem_num_1]),
                (info.rem_cells_2, vec![info.rem_num_2]),
            ])
        })
    }
}
