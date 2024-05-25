use std::io;

use crossterm::{
    cursor::Hide,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use sudoku::Game;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;

    let mut game = Game::new();

    stdout.execute(Hide)?.execute(EnterAlternateScreen)?;
    game.run()?;
    stdout.execute(LeaveAlternateScreen)?;
    
    disable_raw_mode()?;
    
    Ok(())
}
