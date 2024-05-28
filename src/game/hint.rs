use super::utils::{block_idx_2_coord, coord_2_block};

pub fn hidden_single(board: &[[i8; 9]; 9]) -> Option<((usize, usize, i8), String)> {
    let mut row = [[false; 10]; 9];
    let mut col = [[false; 10]; 9];
    let mut block = [[false; 10]; 9];
    for r in 0..9 {
        for c in 0..9 {
            row[r][board[r][c] as usize] = true;
            col[c][board[r][c] as usize] = true;
            block[coord_2_block(r as i8, c as i8) as usize][board[r][c] as usize] = true;
        }
    }

    for r in 0..9 {
        for num in 1..10 {
            if !row[r][num] {
                let mut cur_viable_options = vec![];
                for c in 0..9 {
                    if board[r][c] == 0
                        && !col[c][num]
                        && !block[coord_2_block(r as i8, c as i8) as usize][num]
                    {
                        cur_viable_options.push((r, c, num as i8));
                    }
                }
                // 如果检查到能够确定的格子，直接返回
                if cur_viable_options.len() == 1 {
                    return Some((
                        cur_viable_options[0],
                        format!("第{}行的{}只能填在此格", r + 1, num),
                    ));
                }
            }
        }
    }
    for c in 0..9 {
        for num in 1..10 {
            if !col[c][num] {
                let mut cur_viable_options = vec![];
                for r in 0..9 {
                    if board[r][c] == 0
                        && !row[r][num]
                        && !block[coord_2_block(r as i8, c as i8) as usize][num]
                    {
                        cur_viable_options.push((r, c, num as i8));
                    }
                }
                // 如果检查到能够确定的格子，直接返回
                if cur_viable_options.len() == 1 {
                    return Some((
                        cur_viable_options[0],
                        format!("第{}列的{}只能填在此格", c + 1, num),
                    ));
                }
            }
        }
    }
    for b in 0..9 {
        for num in 1..10 {
            if !block[b][num] {
                let mut cur_viable_options = vec![];
                for idx_in_b in 0..9 {
                    let (r, c) = block_idx_2_coord(b, idx_in_b);
                    if board[r][c] == 0 && !row[r][num] && !col[c][num] {
                        cur_viable_options.push((r, c, num as i8));
                    }
                }
                // 如果检查到能够确定的格子，直接返回
                if cur_viable_options.len() == 1 {
                    return Some((
                        cur_viable_options[0],
                        format!("第{}宫的{}只能填在此格", b + 1, num),
                    ));
                }
            }
        }
    }
    None
}

pub fn naked_single(board: &[[i8; 9]; 9]) -> Option<((usize, usize, i8), String)> {
    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == 0 {
                let b = coord_2_block(r as i8, c as i8) as usize;
                let mut candidate = ([true; 10], 9);
                for idx in 0..9 {
                    let (r1, c1) = block_idx_2_coord(b, idx);
                    candidate.0[board[idx][c] as usize] = false;
                    candidate.0[board[r][idx] as usize] = false;
                    candidate.0[board[r1][c1] as usize] = false;
                }
                candidate.1 = 0;
                let mut num = 0;
                for i in 1..=9 {
                    candidate.1 += candidate.0[i] as i8;
                    if candidate.0[i] {
                        num = i as i8;
                    }
                }
                if (candidate.1 as usize) == 1 {
                    return Some(((r, c, num), format!("此格只能填{}", num)));
                }
            }
        }
    }
    None
}
