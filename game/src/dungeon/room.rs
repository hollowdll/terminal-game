use crossterm::{cursor, execute};
use std::io;

pub fn display_start_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("|    | S |    |");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;
    println!("+-------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 7))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 8))?;

    Ok(start_column + 8)
}

pub fn display_boss_entrance_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("+-------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|    | B |    |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 7))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 8))?;

    Ok(start_column + 8)
}

pub fn display_boss_room(start_column: u16, next_floor: u32) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("+--------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|    | F{} |    |", next_floor);
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("|              |");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("|              |");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("|              |");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;
    println!("+----      ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 8))?;

    Ok(start_column + 8)
}

pub fn display_two_way_up_down_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 7))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 8))?;

    Ok(start_column + 8)
}

pub fn display_two_way_left_right_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("+-------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("               ");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("               ");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;
    println!("+-------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 7))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 8))?;

    Ok(start_column + 8)
}

pub fn display_two_way_up_left_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("              |");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("              |");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;
    println!("+-------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 7))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 8))?;

    Ok(start_column + 8)
}

pub fn display_two_way_up_right_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("|              ");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("|              ");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;
    println!("+-------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 7))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 8))?;

    Ok(start_column + 8)
}

pub fn display_two_way_down_left_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("+-------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("              |");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("              |");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 7))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 8))?;

    Ok(start_column + 8)
}

pub fn display_two_way_down_right_room(start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("+-------------+");
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
    println!("|              ");
    execute!(stdout, cursor::MoveTo(0, start_column + 4))?;
    println!("|              ");
    execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
    println!("|             |");
    execute!(stdout, cursor::MoveTo(0, start_column + 6))?;
    println!("+----     ----+");
    execute!(stdout, cursor::MoveTo(0, start_column + 7))?;
    println!("");
    execute!(stdout, cursor::MoveTo(0, start_column + 8))?;

    Ok(start_column + 8)
}
