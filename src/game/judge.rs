use super::utils::coord_2_block;

// 返回 (
//     board是否为有效的部分解
//     board是否为有效的完全解
//     board中违反约束的格子
// )
pub fn judge_sudoku(board: &[[i8; 9]; 9]) -> (bool, bool, [[bool; 9]; 9]) {
    let mut row: [[(i8, i8); 10]; 9] = [[(-1, -1); 10]; 9];
    let mut col: [[(i8, i8); 10]; 9] = [[(-1, -1); 10]; 9];
    let mut block: [[(i8, i8); 10]; 9] = [[(-1, -1); 10]; 9];
    let mut valid = true;
    let mut full = true;
    let mut valid_cond = [[true; 9]; 9];
    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] > 0 {
                let b = coord_2_block(r as i8, c as i8) as usize;
                if row[r][board[r][c] as usize] != (-1, -1) {
                    valid = false;
                    valid_cond[r][c] = false;
                    let (r1, c1) = row[r][board[r][c] as usize];
                    valid_cond[r1 as usize][c1 as usize] = false;
                }
                if col[c][board[r][c] as usize] != (-1, -1) {
                    valid = false;
                    valid_cond[r][c] = false;
                    let (r1, c1) = col[c][board[r][c] as usize];
                    valid_cond[r1 as usize][c1 as usize] = false;
                }
                if block[b][board[r][c] as usize] != (-1, -1) {
                    valid = false;
                    valid_cond[r][c] = false;
                    let (r1, c1) = block[b][board[r][c] as usize];
                    valid_cond[r1 as usize][c1 as usize] = false;
                }
                row[r][board[r][c] as usize] = (r as i8, c as i8);
                col[c][board[r][c] as usize] = (r as i8, c as i8);
                block[b][board[r][c] as usize] = (r as i8, c as i8);
            } else {
                full = false;
            }
        }
    }
    (valid, valid && full, valid_cond)
}
