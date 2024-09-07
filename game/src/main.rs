use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use game::{
    config::GameConfig,
    dungeon,
    game_data::{create_savefile_if_not_exists, load_save_file},
    main_menu::main_menu,
    session::Player,
};
use std::io;

fn main() -> io::Result<()> {
    if let Err(e) = run() {
        let mut stdout = io::stdout();
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
    execute!(stdout, EnterAlternateScreen, Hide)?;

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

    let dungeon = dungeon::generate_random_dungeon_floor(1);
    dungeon.pretty_print();

    Ok(())
}
