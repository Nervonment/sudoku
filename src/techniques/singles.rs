use crate::{
    state::{
        State, TrackingCandidateCountOfCell, TrackingCandidates, TrackingCellCountOfCandidate,
    },
    utils::block_idx_2_coord,
};

use super::{Direct, DirectOption, House, Technique};

fn hidden_single<T, F1, F2>(
    state: &T,
    cell_cnt_of_candidate: F1,
    coord_transform: F2,
) -> Option<(usize, usize, i8, usize)>
where
    T: State + TrackingCandidates,
    F1: Fn(&T, usize, i8) -> i8,
    F2: Fn(usize, usize) -> (usize, usize),
{
    for i in 0..9 {
        for num in 1..=9 {
            if cell_cnt_of_candidate(state, i, num) == 1 {
                let j = (0..9)
                    .filter(|j: &usize| {
                        let (r, c) = coord_transform(i, *j);
                        state.is_cell_empty(r, c) && state.is_candidate_of(r, c, num)
                    })
                    .next()
                    .unwrap();
                let (r, c) = coord_transform(i, j);
                return Some((r, c, num, i));
            }
        }
    }
    None
}

#[derive(Clone, Copy, Debug)]
pub struct HiddenSingleInfo {
    pub house: House,
    pub fillable: (usize, usize, i8),
}

#[derive(Default)]
pub struct HiddenSingle(pub Option<HiddenSingleInfo>);
impl<T> Technique<T> for HiddenSingle
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn analyze(&mut self, state: &T) {
        self.0 = hidden_single(
            state,
            |p, b, num| p.cell_cnt_of_candidate_in_blk(b, num),
            block_idx_2_coord,
        )
        .map(|res| HiddenSingleInfo {
            house: House::Block(res.3),
            fillable: (res.0, res.1, res.2),
        });
        if self.0.is_none() {
            self.0 = hidden_single(
                state,
                |p, r, num| p.cell_cnt_of_candidate_in_row(r, num),
                |r, c| (r, c),
            )
            .map(|res| HiddenSingleInfo {
                house: House::Row(res.3),
                fillable: (res.0, res.1, res.2),
            });
            if self.0.is_none() {
                self.0 = hidden_single(
                    state,
                    |p, c, num| p.cell_cnt_of_candidate_in_col(c, num),
                    |r, c| (c, r),
                )
                .map(|res| HiddenSingleInfo {
                    house: House::Column(res.3),
                    fillable: (res.0, res.1, res.2),
                });
            }
        }
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        self.0.map(|_| 1.5)
    }
}
impl<T> Direct<T> for HiddenSingle
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn option(&self) -> Option<DirectOption> {
        self.0
            .map(|info| DirectOption(info.fillable.0, info.fillable.1, info.fillable.2))
    }
}

#[derive(Default)]
pub struct HiddenSingleRow(pub Option<HiddenSingleInfo>);
impl<T> Technique<T> for HiddenSingleRow
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn analyze(&mut self, state: &T) {
        self.0 = hidden_single(
            state,
            |p, r, num| p.cell_cnt_of_candidate_in_row(r, num),
            |r, c| (r, c),
        )
        .map(|res| HiddenSingleInfo {
            house: House::Row(res.3),
            fillable: (res.0, res.1, res.2),
        });
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        self.0.map(|_| 1.5)
    }
}
impl<T> Direct<T> for HiddenSingleRow
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn option(&self) -> Option<DirectOption> {
        self.0
            .map(|info| DirectOption(info.fillable.0, info.fillable.1, info.fillable.2))
    }
}

#[derive(Default)]
pub struct HiddenSingleColumn(pub Option<HiddenSingleInfo>);
impl<T> Technique<T> for HiddenSingleColumn
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn analyze(&mut self, state: &T) {
        self.0 = hidden_single(
            state,
            |p, c, num| p.cell_cnt_of_candidate_in_col(c, num),
            |c, r| (r, c),
        )
        .map(|res| HiddenSingleInfo {
            house: House::Column(res.3),
            fillable: (res.0, res.1, res.2),
        });
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        self.0.map(|_| 1.5)
    }
}
impl<T> Direct<T> for HiddenSingleColumn
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn option(&self) -> Option<DirectOption> {
        self.0
            .map(|info| DirectOption(info.fillable.0, info.fillable.1, info.fillable.2))
    }
}

#[derive(Default)]
pub struct HiddenSingleBlock(pub Option<HiddenSingleInfo>);
impl<T> Technique<T> for HiddenSingleBlock
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn analyze(&mut self, state: &T) {
        self.0 = hidden_single(
            state,
            |p, b, num| p.cell_cnt_of_candidate_in_blk(b, num),
            block_idx_2_coord,
        )
        .map(|res| HiddenSingleInfo {
            house: House::Block(res.3),
            fillable: (res.0, res.1, res.2),
        });
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        self.0.map(|_| 1.2)
    }
}
impl<T> Direct<T> for HiddenSingleBlock
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn option(&self) -> Option<DirectOption> {
        self.0
            .map(|info| DirectOption(info.fillable.0, info.fillable.1, info.fillable.2))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NakedSingleInfo(pub (usize, usize, i8));

#[derive(Default)]
pub struct NakedSingle(pub Option<NakedSingleInfo>);
impl<T> Technique<T> for NakedSingle
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
{
    fn analyze(&mut self, state: &T) {
        self.0 = (|| {
            for r in 0..9 {
                for c in 0..9 {
                    if state.is_cell_empty(r, c) && state.candidate_cnt_of_cell(r, c) == 1 {
                        let num = (1..=9)
                            .filter(|num| state.is_candidate_of(r, c, *num))
                            .next()
                            .unwrap();
                        return Some(NakedSingleInfo((r, c, num)));
                    }
                }
            }
            None
        })()
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        self.0.map(|_| 2.3)
    }
}
impl<T> Direct<T> for NakedSingle
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
{
    fn option(&self) -> Option<DirectOption> {
        self.0
            .map(|info| DirectOption(info.0 .0, info.0 .1, info.0 .2))
    }
}
