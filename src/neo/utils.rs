pub fn coord_2_block(r: usize, c: usize) -> usize {
    r / 3 * 3 + c / 3
}

pub fn coord_2_block_idx(r: usize, c: usize) -> (usize, usize) {
    (r / 3 * 3 + c / 3, r % 3 * 3 + c % 3)
}

pub fn block_idx_2_coord(b: usize, bidx: usize) -> (usize, usize) {
    (b / 3 * 3 + bidx / 3, b % 3 * 3 + bidx % 3)
}
