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