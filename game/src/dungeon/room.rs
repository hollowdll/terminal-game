use crossterm::{cursor, execute};
use std::io;

pub fn display_start_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("|    | S |    |");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("+-------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;

    Ok(start_column + 6)
}

pub fn display_twowayupdown_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;

    Ok(start_column + 6)
}
