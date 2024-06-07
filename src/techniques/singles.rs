use crate::{
    state::{
        State, TrackingCandidateCountOfCell, TrackingCandidates, TrackingCellCountOfCandidate,
    },
    utils::block_idx_2_coord,
};

use super::{Direct, House, Technique};

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

pub struct HiddenSingleRow(pub Option<HiddenSingleInfo>);
impl<T> Technique<T> for HiddenSingleRow
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn check(state: &T) -> Self {
        Self(
            hidden_single(
                state,
                |p, r, num| p.cell_cnt_of_candidate_in_row(r, num),
                |r, c| (r, c),
            )
            .map(|res| HiddenSingleInfo {
                house: House::Row(res.3),
                fillable: (res.0, res.1, res.2),
            }),
        )
    }
    fn score() -> f32 {
        1.5
    }
}
impl Into<Option<(usize, usize, i8)>> for HiddenSingleRow {
    fn into(self) -> Option<(usize, usize, i8)> {
        self.0.map(|info| (info.fillable))
    }
}
impl<T> Direct<T> for HiddenSingleRow where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate
{
}

pub struct HiddenSingleColumn(pub Option<HiddenSingleInfo>);
impl<T> Technique<T> for HiddenSingleColumn
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn check(state: &T) -> Self {
        Self(
            hidden_single(
                state,
                |p, c, num| p.cell_cnt_of_candidate_in_col(c, num),
                |c, r| (r, c),
            )
            .map(|res| HiddenSingleInfo {
                house: House::Column(res.3),
                fillable: (res.0, res.1, res.2),
            }),
        )
    }
    fn score() -> f32 {
        1.5
    }
}
impl Into<Option<(usize, usize, i8)>> for HiddenSingleColumn {
    fn into(self) -> Option<(usize, usize, i8)> {
        self.0.map(|info| (info.fillable))
    }
}
impl<T> Direct<T> for HiddenSingleColumn where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate
{
}

pub struct HiddenSingleBlock(pub Option<HiddenSingleInfo>);
impl<T> Technique<T> for HiddenSingleBlock
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn check(state: &T) -> Self {
        Self(
            hidden_single(
                state,
                |p, b, num| p.cell_cnt_of_candidate_in_blk(b, num),
                block_idx_2_coord,
            )
            .map(|res| HiddenSingleInfo {
                house: House::Column(res.3),
                fillable: (res.0, res.1, res.2),
            }),
        )
    }
    fn score() -> f32 {
        1.2
    }
}
impl Into<Option<(usize, usize, i8)>> for HiddenSingleBlock {
    fn into(self) -> Option<(usize, usize, i8)> {
        self.0.map(|info| (info.fillable))
    }
}
impl<T> Direct<T> for HiddenSingleBlock where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate
{
}

#[derive(Clone, Copy, Debug)]
pub struct NakedSingleInfo(pub (usize, usize, i8));
pub struct NakedSingle(pub Option<NakedSingleInfo>);
impl<T> Technique<T> for NakedSingle
where
    T: State + TrackingCandidates + TrackingCandidateCountOfCell,
{
    fn check(state: &T) -> Self {
        Self(
            (|| {
                for r in 0..9 {
                    for c in 0..9 {
                        if state.is_cell_empty(r, c) && state.candidate_cnt_of_cell(r, c) == 1 {
                            let num = (1..=9)
                                .filter(|num| state.is_candidate_of(r, c, *num))
                                .next()
                                .unwrap();
                            return Some((r, c, num));
                        }
                    }
                }
                None
            })()
            .map(|res| NakedSingleInfo(res)),
        )
    }
    fn score() -> f32 {
        2.3
    }
}
impl Into<Option<(usize, usize, i8)>> for NakedSingle {
    fn into(self) -> Option<(usize, usize, i8)> {
        self.0.map(|info| info.0)
    }
}
impl<T> Direct<T> for NakedSingle where T: State + TrackingCandidates + TrackingCandidateCountOfCell {}
