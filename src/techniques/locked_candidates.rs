use crate::{
    state::{State, TrackingCandidates, TrackingCellCountOfCandidate},
    utils::{block_idx_2_coord, coord_2_block},
};

use super::{House, ReducingCandidates, Technique};

#[derive(Clone, Debug)]
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
    fn score() -> f32 {
        2.6
    }
}
impl Into<Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>>> for Pointing {
    fn into(self) -> Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>> {
        self.0
            .clone()
            .map(|info| (vec![(info.rem_cells, vec![info.rem_num])]))
    }
}
impl<T> ReducingCandidates<T> for Pointing where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate
{
}

// TODO: Claiming
