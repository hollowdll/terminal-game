use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use game::{
    config::GameConfig,
    game_data::{create_savefile_if_not_exists, load_save_file},
    main_menu::main_menu,
    session::Player,
};
use std::io;

fn main() -> io::Result<()> {
    run()?;
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
    loop {
        if let Ok(rerender) = main_menu(&mut player, &cfg) {
            if !rerender {
                break;
            }
        } else {
            break;
        }
    }
    disable_raw_mode()?;
    Ok(())
}
