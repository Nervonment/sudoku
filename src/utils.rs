pub fn coord_2_block(r: usize, c: usize) -> usize {
    r / 3 * 3 + c / 3
}

pub fn coord_2_block_idx(r: usize, c: usize) -> (usize, usize) {
    (r / 3 * 3 + c / 3, r % 3 * 3 + c % 3)
}

pub fn block_idx_2_coord(b: usize, bidx: usize) -> (usize, usize) {
    (b / 3 * 3 + bidx / 3, b % 3 * 3 + bidx % 3)
}

pub fn count_one(mut bits: usize) -> usize {
    bits = (bits & 0x5555555555555555usize) + ((bits >> 1) & 0x5555555555555555usize);
    bits = (bits & 0x3333333333333333usize) + ((bits >> 2) & 0x3333333333333333usize);
    bits = (bits & 0x0F0F0F0F0F0F0F0Fusize) + ((bits >> 4) & 0x0F0F0F0F0F0F0F0Fusize);
    bits = (bits & 0x00FF00FF00FF00FFusize) + ((bits >> 8) & 0x00FF00FF00FF00FFusize);
    bits = (bits & 0x0000FFFF0000FFFFusize) + ((bits >> 16) & 0x0000FFFF0000FFFFusize);
    bits = (bits & 0x00000000FFFFFFFFusize) + ((bits >> 32) & 0x00000000FFFFFFFFusize);
    bits
}

pub fn overlap_region(
    (h1, h_idx1): (usize, usize),
    (h2, h_idx2): (usize, usize),
) -> Vec<(usize, usize)> {
    // 0 stands for row, 1 for column and 2 for block
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
                if h_idx2 % 3 == h_idx1 / 3 {
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
