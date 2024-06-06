use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Grid(pub [[i8; 9]; 9]);

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self.0.map(|row| {
            row.map(|cell| {
                if cell > 0 {
                    (cell as u8 + 48) as char
                } else {
                    ' '
                }
            })
        });
        writeln!(f, "┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓")?;
        writeln!(
            f,
            "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
            grid[0][0],
            grid[0][1],
            grid[0][2],
            grid[0][3],
            grid[0][4],
            grid[0][5],
            grid[0][6],
            grid[0][7],
            grid[0][8],
        )?;
        writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?;
        writeln!(
            f,
            "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
            grid[1][0],
            grid[1][1],
            grid[1][2],
            grid[1][3],
            grid[1][4],
            grid[1][5],
            grid[1][6],
            grid[1][7],
            grid[1][8],
        )?;
        writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?;
        writeln!(
            f,
            "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
            grid[2][0],
            grid[2][1],
            grid[2][2],
            grid[2][3],
            grid[2][4],
            grid[2][5],
            grid[2][6],
            grid[2][7],
            grid[2][8],
        )?;
        writeln!(f, "┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫")?;
        writeln!(
            f,
            "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
            grid[3][0],
            grid[3][1],
            grid[3][2],
            grid[3][3],
            grid[3][4],
            grid[3][5],
            grid[3][6],
            grid[3][7],
            grid[3][8],
        )?;
        writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?;
        writeln!(
            f,
            "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
            grid[4][0],
            grid[4][1],
            grid[4][2],
            grid[4][3],
            grid[4][4],
            grid[4][5],
            grid[4][6],
            grid[4][7],
            grid[4][8],
        )?;
        writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?;
        writeln!(
            f,
            "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
            grid[5][0],
            grid[5][1],
            grid[5][2],
            grid[5][3],
            grid[5][4],
            grid[5][5],
            grid[5][6],
            grid[5][7],
            grid[5][8],
        )?;
        writeln!(f, "┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫")?;
        writeln!(
            f,
            "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
            grid[6][0],
            grid[6][1],
            grid[6][2],
            grid[6][3],
            grid[6][4],
            grid[6][5],
            grid[6][6],
            grid[6][7],
            grid[6][8],
        )?;
        writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?;
        writeln!(
            f,
            "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
            grid[7][0],
            grid[7][1],
            grid[7][2],
            grid[7][3],
            grid[7][4],
            grid[7][5],
            grid[7][6],
            grid[7][7],
            grid[7][8],
        )?;
        writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?;
        writeln!(
            f,
            "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
            grid[8][0],
            grid[8][1],
            grid[8][2],
            grid[8][3],
            grid[8][4],
            grid[8][5],
            grid[8][6],
            grid[8][7],
            grid[8][8],
        )?;
        write!(f, "┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛")
    }
}
