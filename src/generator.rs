use rand::random;

use super::solver::{Grader, Solver};

pub fn random_sudoku_puzzle<S1, S2, T>(
    min_blank_cnt: i32, // 需要生成的题目最少空格数
    min_difficulty: T,  // 题目最小难度分数
    max_difficulty: T,  // 题目最大难度分数
) -> [[i8; 9]; 9]
where
    S1: Solver + From<[[i8; 9]; 9]>,
    S2: Solver + Grader<T> + From<[[i8; 9]; 9]>,
    T: PartialOrd + From<i8>
{
    loop {
        // 生成随机终局
        let mut puzzle = S1::from([[0; 9]; 9]).any_solution().unwrap();

        let mut dug = 0; // 已经挖掉的空格数
        let mut trace = vec![]; // 挖空历史记录
        trace.reserve(64);
        let failed_try_threshold = 48; // 挖空失败次数阈值，失败次数超过此值会尝试回退

        let trace_back_step = 24; // 回退的步长
        let mut trace_back_cnt = 0; // 回退的次数
        let trace_back_cnt_threshold = 12; // 回退次数阈值，回退次数超过此值会尝试重新生成终局

        let mut difficulty: T = 0.into(); // 搜索函数在此题目上调用的次数

        while trace_back_cnt < trace_back_cnt_threshold
            && !(dug >= min_blank_cnt
                && (difficulty >= min_difficulty && difficulty <= max_difficulty))
        {
            let mut failed_try = 0;
            let step = match dug {
                ..=40 => 3,
                41.. => 1,
            };
            loop {
                // 一次挖 step 个空
                for _ in 0..step {
                    // 随机选取非空格
                    let (mut r, mut c) = (random::<usize>() % 9, random::<usize>() % 9);
                    while puzzle[r][c] == 0 {
                        (r, c) = (random::<usize>() % 9, random::<usize>() % 9);
                    }
                    trace.push((r, c, puzzle[r][c]));
                    puzzle[r][c] = 0;
                }

                // 挖空后，判断是否有唯一解
                let mut solver = S2::from(puzzle);
                if solver.have_unique_solution() {
                    difficulty = solver.difficulty();
                    break;
                }

                // 没有唯一解，填回刚刚挖的空
                failed_try += 1;
                for _ in 0..step {
                    let last = trace.pop();
                    if last.is_some() {
                        let (r, c, num) = last.unwrap();
                        puzzle[r][c] = num;
                    }
                }

                // 尝试失败次数过多时，退回一定步数重新尝试
                if failed_try > failed_try_threshold {
                    for _ in 0..trace_back_step {
                        let last = trace.pop();
                        if last.is_some() {
                            let (r, c, num) = last.unwrap();
                            puzzle[r][c] = num;
                        }
                    }
                    dug -= trace_back_step;
                    failed_try = 0;
                    trace_back_cnt += 1;
                }
            }
            dug += step;
        }
        if dug >= min_blank_cnt && (difficulty >= min_difficulty && difficulty <= max_difficulty) {
            return puzzle;
        }
    }
}
