pub fn coord_2_block(row: i8, col: i8) -> i8 {
    row / 3 * 3 + col / 3
}

pub fn next_blank(mut row: i8, mut col: i8, board: &[[i8; 9]; 9]) -> Option<(i8, i8)> {
    while row < 9 && board[row as usize][col as usize] > 0 {
        if col == 8 {
            col = 0;
            row += 1;
        } else {
            col += 1
        }
    }
    if row == 9 {
        return None;
    }
    Some((row, col))
}
