use crate::{
    state::{State, TrackingCandidates, TrackingCellCountOfCandidate},
    utils::{block_idx_2_coord, coord_2_block},
};

use super::{House, ReducingCandidates, ReducingCandidatesOption, Technique};

#[derive(Debug)]
pub struct PointingInfo {
    pub block: usize,
    pub rem_house: House,
    pub rem_num: i8,
    pub rem_cells: Vec<(usize, usize)>,
}
pub struct Pointing(pub Option<PointingInfo>);
impl<T> Technique<T> for Pointing
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn check(state: &T) -> Self {
        for b in 0..9 {
            for num in 1..=9 {
                let cnt = state.cell_cnt_of_candidate_in_blk(b, num);
                if cnt < 1 || cnt > 3 {
                    continue;
                }
                let mut bidxs = (0..9).filter(|bidx| {
                    let (r, c) = block_idx_2_coord(b, *bidx);
                    state.is_cell_empty(r, c) && state.is_candidate_of(r, c, num)
                });
                let bidx0 = bidxs.next().unwrap();
                // 在同一行
                if bidxs.clone().all(|bidx| bidx / 3 == bidx0 / 3) {
                    let r = block_idx_2_coord(b, bidx0).0;
                    // 移除同一行中不在这一宫的其他格子候选数中的 num
                    let removes: Vec<(usize, usize)> = (0..9)
                        .filter(|c| {
                            coord_2_block(r, *c) != b
                                && state.is_cell_empty(r, *c)
                                && state.is_candidate_of(r, *c, num)
                        })
                        .map(|c| (r, c))
                        .collect();
                    if !removes.is_empty() {
                        return Pointing(Some(PointingInfo {
                            block: b,
                            rem_house: House::Row(r),
                            rem_num: num,
                            rem_cells: removes,
                        }));
                    }
                }
                // 在同一列
                else if bidxs.all(|bidx| bidx % 3 == bidx0 % 3) {
                    let c = block_idx_2_coord(b, bidx0).1;
                    // 移除同一列中不在这一宫的其他格子候选数中的 num
                    let removes: Vec<(usize, usize)> = (0..9)
                        .filter(|r| {
                            coord_2_block(*r, c) != b
                                && state.is_cell_empty(*r, c)
                                && state.is_candidate_of(*r, c, num)
                        })
                        .map(|r| (r, c))
                        .collect();
                    if !removes.is_empty() {
                        return Pointing(Some(PointingInfo {
                            block: b,
                            rem_house: House::Column(c),
                            rem_num: num,
                            rem_cells: removes,
                        }));
                    }
                }
            }
        }
        Pointing(None)
    }
    fn score(&self) -> f32 {
        2.6
    }
}
impl Into<Option<ReducingCandidatesOption>> for Pointing {
    fn into(self) -> Option<ReducingCandidatesOption> {
        self.0
            .map(|info| ReducingCandidatesOption(vec![(info.rem_cells, vec![info.rem_num])]))
    }
}
impl<T> ReducingCandidates<T> for Pointing where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate
{
}

#[derive(Debug)]
pub struct ClaimingInfo {
    pub house: House,
    pub rem_block: usize,
    pub rem_num: i8,
    pub rem_cells: Vec<(usize, usize)>,
}
pub struct Claiming(pub Option<ClaimingInfo>);
impl<T> Technique<T> for Claiming
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn check(state: &T) -> Self {
        for r in 0..9 {
            for num in 1..=9 {
                let cnt = state.cell_cnt_of_candidate_in_row(r, num);
                if cnt < 1 || cnt > 3 {
                    continue;
                }
                let mut cs = (0..9)
                    .filter(|c| state.is_cell_empty(r, *c) && state.is_candidate_of(r, *c, num));
                let c0 = cs.next().unwrap();
                // 在同一宫
                if cs.all(|c| coord_2_block(r, c) == coord_2_block(r, c0)) {
                    let b = coord_2_block(r, c0);
                    // 移除同一宫中不在这一行的其他格子候选数中的 num
                    let removes: Vec<(usize, usize)> = (0..9)
                        .filter(|bidx| {
                            let (r1, c1) = block_idx_2_coord(b, *bidx);
                            r1 != r
                                && state.is_cell_empty(r1, c1)
                                && state.is_candidate_of(r1, c1, num)
                        })
                        .map(|bidx| block_idx_2_coord(b, bidx))
                        .collect();
                    if !removes.is_empty() {
                        return Claiming(Some(ClaimingInfo {
                            house: House::Row(r),
                            rem_block: b,
                            rem_num: num,
                            rem_cells: removes,
                        }));
                    }
                }
            }
        }

        for c in 0..9 {
            for num in 1..=9 {
                let cnt = state.cell_cnt_of_candidate_in_col(c, num);
                if cnt < 1 || cnt > 3 {
                    continue;
                }
                let mut rs = (0..9)
                    .filter(|r| state.is_cell_empty(*r, c) && state.is_candidate_of(*r, c, num));
                let r0 = rs.next().unwrap();
                // 在同一宫
                if rs.all(|r| coord_2_block(r, c) == coord_2_block(r0, c)) {
                    let b = coord_2_block(r0, c);
                    // 移除同一宫中不在这一列的其他格子候选数中的 num
                    let removes: Vec<(usize, usize)> = (0..9)
                        .filter(|bidx| {
                            let (r1, c1) = block_idx_2_coord(b, *bidx);
                            c1 != c
                                && state.is_cell_empty(r1, c1)
                                && state.is_candidate_of(r1, c1, num)
                        })
                        .map(|bidx| block_idx_2_coord(b, bidx))
                        .collect();
                    if !removes.is_empty() {
                        return Claiming(Some(ClaimingInfo {
                            house: House::Column(c),
                            rem_block: b,
                            rem_num: num,
                            rem_cells: removes,
                        }));
                    }
                }
            }
        }
        Claiming(None)
    }
    fn score(&self) -> f32 {
        2.8
    }
}
impl Into<Option<ReducingCandidatesOption>> for Claiming {
    fn into(self) -> Option<ReducingCandidatesOption> {
        self.0
            .map(|info| ReducingCandidatesOption(vec![(info.rem_cells, vec![info.rem_num])]))
    }
}
impl<T> ReducingCandidates<T> for Claiming where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate
{
}
