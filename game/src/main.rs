use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use game::{
    config::GameConfig,
    game_data::{create_savefile_if_not_exists, load_save_file},
    main_menu::main_menu,
};
use std::io;

fn main() -> io::Result<()> {
    run()?;
    Ok(())
}

fn run() -> io::Result<()> {
    let _config = GameConfig::new();
    create_savefile_if_not_exists()?;
    let _game_data = load_save_file()?;

    enable_raw_mode()?;
    loop {
        if let Ok(rerender) = main_menu() {
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
