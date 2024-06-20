use crate::{
    state::{
        State, TrackingCandidateCountOfCell, TrackingCandidates, TrackingCellCountOfCandidate,
    },
    utils::{block_idx_2_coord, count_one},
};

use super::{House, Technique};

fn overlap_region(
    (h1, h_idx1): (usize, usize),
    (h2, h_idx2): (usize, usize),
) -> Vec<(usize, usize)> {
    match h1 {
        0 => match h2 {
            0 => {
                if h_idx1 == h_idx2 {
                    (0..9).map(|c| (h_idx1, c)).collect()
                } else {
                    vec![]
                }
            }
            1 => vec![(h_idx1, h_idx2)],
            2 => {
                if h_idx2 / 3 == h_idx1 / 3 {
                    (0..3).map(|c0| (h_idx1, c0 + h_idx2 % 3 * 3)).collect()
                } else {
                    vec![]
                }
            }
            _ => vec![],
        },
        1 => match h2 {
            0 => overlap_region((h2, h_idx2), (h1, h_idx1)),
            1 => {
                if h_idx1 == h_idx2 {
                    (0..9).map(|r| (r, h_idx1)).collect()
                } else {
                    vec![]
                }
            }
            2 => {
                if h_idx2 % 3 == h_idx1 % 3 {
                    (0..3).map(|r0| (r0 + h_idx2 / 3 * 3, h_idx1)).collect()
                } else {
                    vec![]
                }
            }
            _ => vec![],
        },
        2 => match h2 {
            0..=1 => overlap_region((h2, h_idx2), (h1, h_idx1)),
            2 => {
                if h_idx1 == h_idx2 {
                    (0..9).map(|bidx| block_idx_2_coord(h_idx1, bidx)).collect()
                } else {
                    vec![]
                }
            }
            _ => vec![],
        },
        _ => vec![],
    }
}

pub fn fish<T>(
    state: &T,
    size: usize,
    base_row_count: usize,
    base_col_count: usize,
    base_blk_count: usize,
    cover_row_count: usize,
    cover_col_count: usize,
    cover_blk_count: usize,
) -> 
bool
// Option<FishInfo>
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    let row_count = base_row_count + cover_row_count;
    let col_count = base_col_count + cover_col_count;
    let blk_count = base_blk_count + cover_blk_count;

    let coord_transforms = [|r, c| (r, c), |c, r| (r, c), block_idx_2_coord];
    let cell_cnt_of_candidate: [&dyn Fn(&T, usize, i8) -> i8; 3] = [
        &TrackingCellCountOfCandidate::cell_cnt_of_candidate_in_row,
        &TrackingCellCountOfCandidate::cell_cnt_of_candidate_in_col,
        &TrackingCellCountOfCandidate::cell_cnt_of_candidate_in_blk,
    ];

    const TWO_TO_9: usize = 1 << 9;
    for num in 1..=9 {
        for rows_bitmap in 0..TWO_TO_9 {
            if count_one(rows_bitmap) == row_count {
                for cols_bitmap in 0..TWO_TO_9 {
                    if count_one(cols_bitmap) == col_count {
                        for blks_bitmap in 0..TWO_TO_9 {
                            if count_one(blks_bitmap) == blk_count {
                                for base_rows_bitmap in 0..TWO_TO_9 {
                                    if (base_rows_bitmap & !rows_bitmap) == 0
                                        && count_one(base_rows_bitmap) == base_row_count
                                    {
                                        let cover_rows_bitmap = rows_bitmap & !base_rows_bitmap;
                                        assert!(count_one(cover_rows_bitmap) == cover_row_count);
                                        for base_cols_bitmap in 0..TWO_TO_9 {
                                            if (base_cols_bitmap & !cols_bitmap) == 0
                                                && count_one(base_cols_bitmap) == base_col_count
                                            {
                                                let cover_cols_bitmap =
                                                    cols_bitmap & !base_cols_bitmap;
                                                assert!(
                                                    count_one(cover_cols_bitmap) == cover_col_count
                                                );
                                                'outer: for base_blks_bitmap in 0..TWO_TO_9 {
                                                    if (base_blks_bitmap & !blks_bitmap) == 0
                                                        && count_one(base_blks_bitmap)
                                                            == base_blk_count
                                                    {
                                                        let cover_blks_bitmap =
                                                            blks_bitmap & !base_blks_bitmap;
                                                        assert!(
                                                            count_one(cover_blks_bitmap)
                                                                == cover_blk_count
                                                        );

                                                        let base_houses: Vec<(usize, usize)> = (0
                                                            ..9)
                                                            .filter_map(|r| {
                                                                if (1 << r) & base_rows_bitmap != 0
                                                                {
                                                                    Some((0, r))
                                                                } else {
                                                                    None
                                                                }
                                                            })
                                                            .chain((0..9).filter_map(|c| {
                                                                if (1 << c) & base_cols_bitmap != 0
                                                                {
                                                                    Some((1, c))
                                                                } else {
                                                                    None
                                                                }
                                                            }))
                                                            .chain((0..9).filter_map(|b| {
                                                                if (1 << b) & base_blks_bitmap != 0
                                                                {
                                                                    Some((2, b))
                                                                } else {
                                                                    None
                                                                }
                                                            }))
                                                            .collect();
                                                        let cover_houses: Vec<(usize, usize)> = (0
                                                            ..9)
                                                            .filter_map(|r| {
                                                                if (1 << r) & cover_rows_bitmap != 0
                                                                {
                                                                    Some((0, r))
                                                                } else {
                                                                    None
                                                                }
                                                            })
                                                            .chain((0..9).filter_map(|c| {
                                                                if (1 << c) & cover_cols_bitmap != 0
                                                                {
                                                                    Some((1, c))
                                                                } else {
                                                                    None
                                                                }
                                                            }))
                                                            .chain((0..9).filter_map(|b| {
                                                                if (1 << b) & cover_blks_bitmap != 0
                                                                {
                                                                    Some((2, b))
                                                                } else {
                                                                    None
                                                                }
                                                            }))
                                                            .collect();

                                                        for cover_house in &cover_houses {
                                                            if cell_cnt_of_candidate[cover_house.0](
                                                                state,
                                                                cover_house.1,
                                                                num,
                                                            ) > 0
                                                            {
                                                                break 'outer;
                                                            }
                                                        }
                                                        for base_house in &base_houses {
                                                            if cell_cnt_of_candidate[base_house.0](
                                                                state,
                                                                base_house.1,
                                                                num,
                                                            ) == 0
                                                            {
                                                                break 'outer;
                                                            }
                                                        }

                                                        for base_house in &base_houses {
                                                            let region: Vec<(usize, usize)> =
                                                                cover_houses
                                                                    .iter()
                                                                    .flat_map(|cover_house| {
                                                                        overlap_region(
                                                                            *base_house,
                                                                            *cover_house,
                                                                        )
                                                                    })
                                                                    .collect();
                                                            for idx_in_h in 0..9 {
                                                                let coord = coord_transforms
                                                                    [base_house.0](
                                                                    base_house.1,
                                                                    idx_in_h,
                                                                );
                                                                if state.is_candidate_of(
                                                                    coord.0, coord.1, num,
                                                                ) && region
                                                                    .iter()
                                                                    .find(|coord1| **coord1 == coord)
                                                                    == None
                                                                {
                                                                    break 'outer;
                                                                }
                                                            }
                                                        }

                                                        println!("-------------------");
                                                        println!("base houses:");
                                                        println!("{:?}", base_houses);
                                                        println!("cover houses:");
                                                        println!("{:?}", cover_houses);
                                                        return true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

#[derive(Clone, Debug)]
pub struct FishInfo {
    size: usize,
    base_set: Vec<House>,
    cover_set: Vec<House>,
    candidate: i8,
}

#[derive(Default)]
pub struct Fish(pub Option<FishInfo>);
impl<T> Technique<T> for Fish
where
    T: State + TrackingCandidates,
{
    fn analyze(&mut self, state: &T) {
        for size in 2..8 {}
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        None
    }
}
