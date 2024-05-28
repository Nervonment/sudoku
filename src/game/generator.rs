use std::time::Instant;

use rand::random;

use crate::game::solver::SudokuSolverH;

use super::solver::{SudokuSolver, SudokuSolverS};

// 尝试生成最多有max_blank_cnt个格子的数独谜面
pub fn random_sudoku_puzzle(max_blank_cnt: i32) -> [[i8; 9]; 9] {
    let mut puzzle = SudokuSolverS::new([[0; 9]; 9]).get_solution().unwrap();
    let begin_time = Instant::now();
    for _ in 0..max_blank_cnt {
        loop {
            // 随机选取非空格
            let (mut r, mut c) = (random::<usize>() % 9, random::<usize>() % 9);
            while puzzle[r][c] == 0 {
                (r, c) = (random::<usize>() % 9, random::<usize>() % 9);
            }
            // 挖空，判断是否有唯一解
            let tmp = puzzle[r][c];
            puzzle[r][c] = 0;
            let mut solver = SudokuSolverH::new(puzzle);
            if solver.have_unique_solution() {
                break;
            }
            puzzle[r][c] = tmp;
            // 如果尝试了很多次以后都没有找到新的空可以挖，
            // 就直接返回
            if begin_time.elapsed().as_millis() > 500 {
                return puzzle;
            }
        }
    }

    puzzle
}

fn random_sudoku_puzzle_1(
    min_blank_cnt: i32,  // 需要生成的题目最少空格数
    min_invoke_cnt: i32, // 搜索函数在此题上最少调用次数，用以衡量题目难度
    max_invoke_cnt: i32, // 搜索函数在此题上最多调用次数
) -> [[i8; 9]; 9] {
    loop {
        // 生成随机终局
        let mut puzzle = SudokuSolverS::new([[0; 9]; 9]).get_solution().unwrap();
        // println!("生成了新的终局，正在尝试在此基础上挖空生成题目...");

        let mut dug = 0; // 已经挖掉的空格数
        let mut trace = vec![]; // 挖空历史记录
        trace.reserve(64);
        let failed_try_threshold = 48; // 挖空失败次数阈值，失败次数超过此值会尝试回退

        let trace_back_step = 24; // 回退的步长
        let mut trace_back_cnt = 0; // 回退的次数
        let trace_back_cnt_threshold = 12; // 回退次数阈值，回退次数超过此值会尝试重新生成终局

        let mut invoke_cnt = -1; // 搜索函数在此题目上调用的次数

        while trace_back_cnt < trace_back_cnt_threshold
            && !(dug >= min_blank_cnt
                && (invoke_cnt >= min_invoke_cnt && invoke_cnt <= max_invoke_cnt))
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
                let mut solver = SudokuSolverH::new(puzzle);
                if solver.have_unique_solution() {
                    invoke_cnt = solver.invoke_cnt;
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
                    // println!(
                    // "挖空失败超过{}次，退回{}步重新尝试挖空",
                    // failed_try_threshold, trace_back_step
                    // );
                }
            }
            dug += step;
        }
        if dug >= min_blank_cnt && (invoke_cnt >= min_invoke_cnt && invoke_cnt <= max_invoke_cnt) {
            return puzzle;
        }
        // println!("这个终局可能不太行，正在换一个终局重新生成...");
    }
}

pub fn random_sudoku_puzzle_extrahard() -> [[i8; 9]; 9] {
    random_sudoku_puzzle_1(40, 2000, 10000)
}

pub fn random_sudoku_puzzle_hard() -> [[i8; 9]; 9] {
    random_sudoku_puzzle_1(50, 200, 2000)
}

pub fn random_sudoku_puzzle_normal() -> [[i8; 9]; 9] {
    random_sudoku_puzzle_1(55, 90, 200)
}

pub fn random_sudoku_puzzle_easy() -> [[i8; 9]; 9] {
    random_sudoku_puzzle_1(50, 50, 90)
}

pub fn random_sudoku_puzzle_extraeasy() -> [[i8; 9]; 9] {
    random_sudoku_puzzle_1(35, 35, 50)
}

pub fn random_sudoku_puzzle_phishing() -> [[i8; 9]; 9] {
    let puzzles: [&str; 50] = [
        "500000300020100070008000009040007000000821000000600010300000800060004020009000005",
        "800000009040001030007000600000023000050904020000105000006000700010300040900000008",
        "000070100000008050020900003530000000062000004094600000000001800300200009000050070",
        "000006080000100200009030005040070003000008010000200600071090000590000004804000000",
        "000056000050109000000020040090040070006000300800000002300000008002000600070500010",
        "500000004080006090001000200070308000000050000000790030002000100060900080400000005",
        "070200009003060000400008000020900010800004000006030000000000600090000051000700002",
        "100080000005900000070002000009500040800010000060007200000000710000004603030000402",
        "000900100000080007004002050200005040000100900000070008605000030300006000070000006",
        "000001080030500200000040006200300500000008010000060004050000700300970000062000000",
        "800000005040003020007000100000004000090702060000639000001000700030200040500000008",
        "900000001030004070006000200050302000000060000000078050002000600040700030100000009",
        "500000008030007040001000900020603000000725000000800060009000100070400030800000005",
        "400000009070008030006000100050702000000065000000003020001000600080300070900000004",
        "100006009007080030000200400000500070300001002000090600060003050004000000900020001",
        "800000001050009040003000600070056000000980000000704020006000300090400050100000008",
        "010000009005080700300700060004250000000000000000840200008007500600000030090000001",
        "300000005020007040001000900080036000000028000000704060009000100070400020500000003",
        "400000003080002060007000900010508000000701000000026050009000700020600080300000004",
        "600005020040700000009080000010000302000000087000200104070400003500006000008090000",
        "007002000500090400010600000400050003060100000002007000000000810900000306000080059",
        "000007090000800400003060001420010000031000002605000000060400800500020006000009070",
        "000600001000020400300009050090005030000040200000100006570008000002000000080000090",
        "006003000900080200070400000003006000040700000800020090500000008000000709000510020",
        "010300000000009000000710050004050900200000006070800030600000002080030070009000400",
        "000008070000300200005040009260094000059000006401000000000200300100060004000007080",
        "000800300000010005004002070200007040000300807000050001907000060600009000050000000",
        "800000007040001030009000600000532000050108020000400000006000900010300040700000008",
        "400000008050002090001000600070503000000060000000790030006000100020900050800000004",
        "300000009010006050002000400070060000000701000000845070004000200060500010900000003",
        "000000789000100036009000010200030000070004000008500100300020000005800090040007000",
        "100000000006700020080030500007060030000500008000004900300800600002090070040000001",
        "700000005040001030002000900060008000000946000000103080009000200010300040500000007",
        "001020000300004000050600070080900005002003000400010000070000038000800069000000200",
        "007580000000030000000076005400000020090000100003060008010600900006800003200000040",
        "097000000301005000045000800003008400000020060000100009700004300000900001000060020",
        "003700000050004000100020080900000012000000400080010090007300000200090006040005000",
        "000000100600000874000007026030400000005090000100008002009050000200001008040300000",
        "100000004020006090005000800030650000000372000000098070008000500060900020400000001",
        "005300000800000020070010500400005300010070006003200080060500009004000030000009700",
        "000002005006700400000009008070090000600400700010000080060300100300000002400005000",
        "020000600400080007009000010005006000300040900010200000000700004000001050800090300",
        "900000007030008040006000200010389000000010000000205010002000600080400030700000009",
        "002400006030010000500008000007000002010000030900600400000007001000090080400200500",
        "100300000020090400005007000800000100040000020007060003000400800000020090006005007",
        "002600000030080000500009100006000002080000030700001400000004005010020080000700900",
        "003500100040080000600009000800000002050700030001000400000006009000020080070100500",
        "300000906040200080000060000050800020009000307000007000010042000000000010508100000",
        "000090050010000030002300700004500070800000200000006400090010000080060000005400007",
        "100500000200000030004060100006007000008000009400080200000009007040010600000005003",
    ];
    let puzzle_str = String::from(puzzles[random::<usize>() % puzzles.len()]);
    let mut puzzle = [[0; 9]; 9];
    for r in 0..9 {
        for c in 0..9 {
            puzzle[r][c] = puzzle_str.as_bytes()[r * 9 + c] as i8 - 48;
        }
    }
    puzzle
}
