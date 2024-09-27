use crossterm::{
    cursor::{Hide, Show},
    execute,
    style::{Color, SetBackgroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io;
use terminal_rpg::{
    config::GameConfig,
    game_data::{create_savefile_if_not_exists, load_save_file},
    menu::main_menu::main_menu,
    session::Player,
    util::reset_background_color,
};

fn main() -> io::Result<()> {
    if let Err(e) = run() {
        let mut stdout = io::stdout();
        reset_background_color()?;
        execute!(stdout, LeaveAlternateScreen, Show, Clear(ClearType::All))?;
        eprintln!("Error: {}", e);
    }
    Ok(())
}

fn run() -> io::Result<()> {
    let cfg = GameConfig::new();
    create_savefile_if_not_exists()?;
    let game_data = match load_save_file() {
        Ok(game_data) => game_data,
        Err(e) => {
            eprintln!("Failed to load save file, it may be corrupted");
            return Err(e);
        }
    };
    let mut player = Player::new(game_data);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        Hide,
        SetBackgroundColor(Color::Black)
    )?;

    loop {
        if let Ok(rerender) = main_menu(&mut player, &cfg) {
            if !rerender {
                break;
            }
        } else {
            break;
        }
    }

    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;

    Ok(())
}
