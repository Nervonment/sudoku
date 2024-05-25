pub mod game;
pub mod ui;

use std::{
    io::{self, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEventKind},
    terminal::Clear,
    ExecutableCommand,
};
use game::{generator::random_sudoku_puzzle, judge::judge_sudoku, utils::next_blank};
use ui::{
    draw_current_grid, draw_grid, draw_instructions, draw_numbers, draw_result, draw_settings,
    draw_titlescreen,
};

#[derive(Debug)]
enum Page {
    TitleScreen,
    Gaming,
    Settings,
}

enum Level {
    Easy,
    Normal,
    Hard,
}

impl Level {
    fn to_string(&self) -> String {
        match self {
            Level::Easy => "简单",
            Level::Normal => "普通",
            Level::Hard => "困难",
        }
        .into()
    }
}

pub struct Game {
    page: Page,
    should_quit: bool,
    level: Level,
}

impl Game {
    pub fn new() -> Self {
        Self {
            page: Page::TitleScreen,
            should_quit: false,
            level: Level::Normal,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        while !self.should_quit {
            match self.page {
                Page::Gaming => self.gaming_page()?,
                Page::TitleScreen => self.titlescreen_page()?,
                Page::Settings => self.settings_page()?,
            }
        }
        Ok(())
    }

    fn titlescreen_page(&mut self) -> io::Result<()> {
        draw_titlescreen()?;
        let mut should_quit = false;
        while !should_quit {
            if poll(Duration::from_millis(20))? {
                match read()? {
                    Event::Key(event) => {
                        if event.kind == KeyEventKind::Press {
                            match event.code {
                                KeyCode::Enter => {
                                    self.page = Page::Gaming;
                                    should_quit = true;
                                }
                                KeyCode::Esc => {
                                    self.should_quit = true;
                                    should_quit = true;
                                }
                                KeyCode::Tab => {
                                    self.page = Page::Settings;
                                    should_quit = true;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn gaming_page(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let mut should_quit = false;
        while !should_quit {
            let blank_cnt = match self.level {
                Level::Easy => 25,
                Level::Normal => 45,
                Level::Hard => 60,
            };
            let puzzle = random_sudoku_puzzle(blank_cnt);
            let mut player_solution = puzzle;
            let mut current_grid = next_blank(0, 0, &puzzle).unwrap();
            let mut valid_cond = [[true; 9]; 9];
            let mut solved = false;

            draw_instructions()?;

            let begin_time = Instant::now();

            while !should_quit && !solved {
                match read()? {
                    Event::Key(event) => {
                        if event.kind == KeyEventKind::Press {
                            match event.code {
                                KeyCode::Char(ch) => {
                                    if ch >= '1'
                                        && ch <= '9'
                                        && puzzle[current_grid.0 as usize][current_grid.1 as usize]
                                            == 0
                                    {
                                        player_solution[current_grid.0 as usize]
                                            [current_grid.1 as usize] = ch as i8 - 48;
                                        (_, solved, valid_cond) = judge_sudoku(player_solution);
                                        if valid_cond[current_grid.0 as usize]
                                            [current_grid.1 as usize]
                                        {
                                            let prev_grid = current_grid;
                                            current_grid = next_blank(
                                                current_grid.0,
                                                current_grid.1,
                                                &player_solution,
                                            )
                                            .unwrap_or(prev_grid);
                                        }
                                    } else {
                                        if puzzle[current_grid.0 as usize][current_grid.1 as usize]
                                            == 0
                                        {
                                            player_solution[current_grid.0 as usize]
                                                [current_grid.1 as usize] = 0;
                                            (_, solved, valid_cond) = judge_sudoku(player_solution);
                                        }
                                    }
                                }
                                KeyCode::Up => {
                                    if current_grid.0 > 0 {
                                        current_grid.0 -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if current_grid.0 < 8 {
                                        current_grid.0 += 1;
                                    }
                                }
                                KeyCode::Left => {
                                    if current_grid.1 > 0 {
                                        current_grid.1 -= 1;
                                    }
                                }
                                KeyCode::Right => {
                                    if current_grid.1 < 8 {
                                        current_grid.1 += 1;
                                    }
                                }
                                KeyCode::Backspace => {
                                    for r in 0..9 {
                                        for c in 0..9 {
                                            if !valid_cond[r][c] && puzzle[r][c] == 0 {
                                                player_solution[r][c] = 0;
                                            }
                                        }
                                    }
                                    (_, solved, valid_cond) = judge_sudoku(player_solution);
                                }
                                KeyCode::Esc => {
                                    should_quit = true;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
                draw_grid()?;
                draw_current_grid(current_grid.0, current_grid.1)?;
                draw_numbers(&puzzle, &player_solution, &valid_cond)?;
                stdout.flush()?;
            }

            if !should_quit {
                stdout.execute(Clear(crossterm::terminal::ClearType::All))?;
                let duration_sec = begin_time.elapsed().as_secs();
                draw_result(duration_sec)?;

                loop {
                    if let Event::Key(e) = read()? {
                        if e.kind == KeyEventKind::Press {
                            if e.code == KeyCode::Enter {
                                break;
                            } else if e.code == KeyCode::Esc {
                                should_quit = true;
                                break;
                            }
                        }
                    }
                    sleep(Duration::from_millis(50));
                }
            }
        }
        self.page = Page::TitleScreen;
        Ok(())
    }

    fn settings_page(&mut self) -> io::Result<()> {
        let mut should_quit = false;
        draw_settings(&self.level.to_string())?;
        while !should_quit {
            if poll(Duration::from_millis(20))? {
                if let Event::Key(e) = read()? {
                    if e.kind == KeyEventKind::Press {
                        match e.code {
                            KeyCode::Left => {
                                self.level = match self.level {
                                    Level::Easy => Level::Easy,
                                    Level::Normal => Level::Easy,
                                    Level::Hard => Level::Normal,
                                };
                                draw_settings(&self.level.to_string())?;
                            }
                            KeyCode::Right => {
                                self.level = match self.level {
                                    Level::Easy => Level::Normal,
                                    Level::Normal => Level::Hard,
                                    Level::Hard => Level::Hard,
                                };
                                draw_settings(&self.level.to_string())?;
                            }
                            KeyCode::Enter => {
                                self.page = Page::TitleScreen;
                                should_quit = true;
                            }
                            KeyCode::Esc => {
                                self.page = Page::TitleScreen;
                                should_quit = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
