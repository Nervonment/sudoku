use std::io;

use crossterm::{
    cursor::MoveTo,
    style::{PrintStyledContent, Stylize},
    terminal::{size, Clear},
    ExecutableCommand, QueueableCommand,
};

pub fn draw_grid() -> io::Result<()> {
    let mut stdout = io::stdout();
    for y in 0..19 {
        for x in 0..37 {
            stdout.queue(MoveTo(x, y))?;
            let ch = match y {
                0 => match x {
                    0 => '┏',
                    36 => '┓',
                    _ => {
                        if x % 12 == 0 {
                            '┳'
                        } else if x % 4 == 0 {
                            '┯'
                        } else {
                            '━'
                        }
                    }
                },
                18 => match x {
                    0 => '┗',
                    36 => '┛',
                    _ => {
                        if x % 12 == 0 {
                            '┻'
                        } else if x % 4 == 0 {
                            '┷'
                        } else {
                            '━'
                        }
                    }
                },
                _ => {
                    if y % 6 == 0 {
                        match x {
                            0 => '┣',
                            36 => '┫',
                            _ => {
                                if x % 12 == 0 {
                                    '╋'
                                } else if x % 4 == 0 {
                                    '┿'
                                } else {
                                    '━'
                                }
                            }
                        }
                    } else if y % 2 == 0 {
                        match x {
                            0 => '┠',
                            36 => '┨',
                            _ => {
                                if x % 12 == 0 {
                                    '╂'
                                } else if x % 4 == 0 {
                                    '┼'
                                } else {
                                    '─'
                                }
                            }
                        }
                    } else {
                        if x % 12 == 0 {
                            '┃'
                        } else if x % 4 == 0 {
                            '│'
                        } else {
                            ' '
                        }
                    }
                }
            };
            stdout.queue(PrintStyledContent(ch.dim()))?;
        }
    }
    Ok(())
}

pub fn draw_current_grid(r: i8, c: i8) -> io::Result<()> {
    let mut stdout = io::stdout();
    let (r, c) = (r as u16, c as u16);
    stdout
        .queue(MoveTo(c * 4, r * 2))?
        .queue(PrintStyledContent("┏━━━┓".yellow()))?
        .queue(MoveTo(c * 4, r * 2 + 1))?
        .queue(PrintStyledContent("┃   ┃".yellow()))?
        .queue(MoveTo(c * 4, r * 2 + 2))?
        .queue(PrintStyledContent("┗━━━┛".yellow()))?;

    Ok(())
}

pub fn draw_numbers(
    puzzle: &[[i8; 9]; 9],
    player_solution: &[[i8; 9]; 9],
    valid_cond: &[[bool; 9]; 9],
) -> io::Result<()> {
    let mut stdout = io::stdout();
    let chs = [' ', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for r in 0..9 {
        for c in 0..9 {
            stdout.queue(MoveTo(c * 4 + 2, r * 2 + 1))?;
            if puzzle[r as usize][c as usize] > 0 {
                stdout.queue(PrintStyledContent(if valid_cond[r as usize][c as usize] {
                    chs[puzzle[r as usize][c as usize] as usize].white().bold()
                } else {
                    chs[puzzle[r as usize][c as usize] as usize].red().bold()
                }))?;
            } else {
                stdout.queue(PrintStyledContent(if valid_cond[r as usize][c as usize] {
                    chs[player_solution[r as usize][c as usize] as usize].yellow()
                } else {
                    chs[player_solution[r as usize][c as usize] as usize]
                        .red()
                        .dim()
                }))?;
            }
        }
    }
    Ok(())
}

pub fn draw_instructions() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(Clear(crossterm::terminal::ClearType::All))?;
    stdout
        .execute(MoveTo(42, 7))?
        .execute(PrintStyledContent("←↓↑→".bold().grey()))?
        .execute(PrintStyledContent(": 移动".white()))?
        .execute(MoveTo(42, 8))?
        .execute(PrintStyledContent("1~9".bold().grey()))?
        .execute(PrintStyledContent(": 填入数字".white()))?
        .execute(MoveTo(42, 9))?
        .execute(PrintStyledContent("空格".bold().grey()))?
        .execute(PrintStyledContent(": 清除当前格".white()))?
        .execute(MoveTo(42, 10))?
        .execute(PrintStyledContent("退格".bold().grey()))?
        .execute(PrintStyledContent(": 清除所有错误格".white()))?
        .execute(MoveTo(42, 11))?
        .execute(PrintStyledContent("Esc".bold().grey()))?
        .execute(PrintStyledContent(": 回到标题画面".white()))?;
    Ok(())
}

pub fn draw_result(duration_sec: u64) -> io::Result<()> {
    let mut stdout = io::stdout();
    let size = size()?;
    stdout
        .execute(MoveTo(size.0 / 2 - 14, size.1 / 2))?
        .execute(PrintStyledContent(
            "你做到了！你把它解出来了！".bold().white(),
        ))?
        .execute(MoveTo(size.0 / 2 - 8, size.1 / 2 + 1))?
        .execute(PrintStyledContent(
            format!("用时: {}分{}秒", duration_sec / 60, duration_sec % 60).white(),
        ))?
        .execute(MoveTo(size.0 / 2 - 10, size.1 / 2 + 3))?
        .execute(PrintStyledContent("按Enter开始新游戏".white()))?;
    Ok(())
}

pub fn draw_titlescreen() -> io::Result<()> {
    let mut stdout = io::stdout();
    let size = size()?;
    stdout
        .execute(Clear(crossterm::terminal::ClearType::All))?
        .execute(MoveTo(size.0 / 2 - 4, size.1 / 2))?
        .execute(PrintStyledContent("🔢 SUDOKU".yellow().bold()))?
        .execute(MoveTo(size.0 / 2 - 4, size.1 / 2 + 1))?
        .execute(PrintStyledContent("🔢 数独".yellow().bold()))?
        .execute(MoveTo(size.0 / 2 - 5, size.1 / 2 + 3))?
        .execute(PrintStyledContent("Enter".bold().grey()))?
        .execute(PrintStyledContent(": 开始".white()))?
        .execute(MoveTo(size.0 / 2 - 5, size.1 / 2 + 4))?
        .execute(PrintStyledContent("Tab".bold().grey()))?
        .execute(PrintStyledContent(": 设置".white()))?
        .execute(MoveTo(size.0 / 2 - 5, size.1 / 2 + 5))?
        .execute(PrintStyledContent("Esc".bold().grey()))?
        .execute(PrintStyledContent(": 退出".white()))?;

    Ok(())
}

pub fn draw_settings(level: &String) -> io::Result<()> {
    let mut stdout = io::stdout();
    let size = size()?;
    stdout
        .execute(Clear(crossterm::terminal::ClearType::All))?
        .execute(MoveTo(size.0 / 2 - 4, size.1 / 2))?
        .execute(PrintStyledContent("🔢 SUDOKU".yellow().bold()))?
        .execute(MoveTo(size.0 / 2 - 4, size.1 / 2 + 1))?
        .execute(PrintStyledContent("🔢 数独".yellow().bold()))?
        .execute(MoveTo(size.0 / 2 - 4, size.1 / 2 + 3))?
        .execute(PrintStyledContent("设置难度".bold().grey()))?
        .execute(MoveTo(size.0 / 2 - 5, size.1 / 2 + 4))?
        .execute(PrintStyledContent(format!("➖ {} ➕", level).white()))?;
    Ok(())
}
