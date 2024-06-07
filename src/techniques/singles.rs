use crate::{
    state::{
        full_state::FullState, State, TrackingCandidateCountOfCell, TrackingCandidates,
        TrackingCellCountOfCandidate,
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

#[derive(Clone, Copy)]
pub struct HiddenSingleInfo {
    pub house: House,
    pub fillable: (usize, usize, i8),
}

pub struct HiddenSingleRow(Option<HiddenSingleInfo>);
impl Technique for HiddenSingleRow {
    fn check(state: &FullState) -> Self {
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
}
impl Direct for HiddenSingleRow {
    fn fillable(&self) -> Option<(usize, usize, i8)> {
        self.0.map(|info| (info.fillable))
    }
}

pub struct HiddenSingleColumn(Option<HiddenSingleInfo>);
impl Technique for HiddenSingleColumn {
    fn check(state: &FullState) -> Self {
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
}
impl Direct for HiddenSingleColumn {
    fn fillable(&self) -> Option<(usize, usize, i8)> {
        self.0.map(|info| (info.fillable))
    }
}

pub struct HiddenSingleBlock(Option<HiddenSingleInfo>);
impl Technique for HiddenSingleBlock {
    fn check(state: &FullState) -> Self {
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
}
impl Direct for HiddenSingleBlock {
    fn fillable(&self) -> Option<(usize, usize, i8)> {
        self.0.map(|info| (info.fillable))
    }
}

#[derive(Clone, Copy)]
pub struct NakedSingleInfo((usize, usize, i8));
pub struct NakedSingle(Option<NakedSingleInfo>);
impl Technique for NakedSingle {
    fn check(state: &FullState) -> Self {
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
}
impl Direct for NakedSingle {
    fn fillable(&self) -> Option<(usize, usize, i8)> {
        self.0.map(|info| info.0)
    }
}
