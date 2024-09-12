use crate::{
    dungeon::{
        generate_random_dungeon_floor,
        room::{display_start_room, display_twowayupdown_room},
        DungeonFloor, RoomCoordinates, RoomKind,
    },
    menu::character::menu_character,
    session::Player,
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

pub struct GameMenuReturnOptions {
    /// If should return to main menu.
    pub main_menu: bool,
    /// If should rerender current menu.
    pub rerender: bool,
}

/// Returns true if should go back to main menu.
pub fn menu_start_dungeon_floor(player: &mut Player) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Start Dungeon Floor", "Return to main menu"];
    let mut selected_index = 0;
    let start_column: u16 = 1;
    let go_back = true;
    let character = match &player.character {
        Some(character) => character,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No selected character",
            ))
        }
    };

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!(
            "Character: {} (Level {}, Dungeon Floor {})",
            character.data.metadata.name,
            character.data.stats.general_stats.character_level,
            character.data.stats.general_stats.current_dungeon_floor
        );
        execute!(stdout, cursor::MoveTo(0, 1))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }

    match menu_items[selected_index] {
        "Start Dungeon Floor" => {
            let mut dungeon_floor = generate_random_dungeon_floor(
                character.data.stats.general_stats.current_dungeon_floor,
            );
            menu_dungeon_floor(&mut dungeon_floor, player, &RoomCoordinates::new(0, 0))?;
        }
        "Return to main menu" => {}
        _ => {}
    }

    Ok(go_back)
}

/// Returns true if should go back to main menu.
pub fn menu_dungeon_floor(
    dungeon_floor: &mut DungeonFloor,
    player: &mut Player,
    current_room_coords: &RoomCoordinates,
) -> io::Result<GameMenuReturnOptions> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let mut menu_items = Vec::new();
    let mut selected_index = 0;
    let mut character = match &mut player.character {
        Some(character) => character,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No selected character",
            ))
        }
    };
    let current_room = match dungeon_floor.rooms.get(current_room_coords) {
        Some(room) => room,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Dungeon Room should exist, but was not found",
            ))
        }
    };

    if let Some(_) = current_room.adjacents.up {
        menu_items.push("Go Up");
    }
    if let Some(_) = current_room.adjacents.down {
        menu_items.push("Go Down");
    }
    if let Some(_) = current_room.adjacents.right {
        menu_items.push("Go Right");
    }
    if let Some(_) = current_room.adjacents.left {
        menu_items.push("Go Left");
    }
    match current_room.kind {
        RoomKind::Start => menu_items.push("Enter Shop"),
        RoomKind::BossEntrance => menu_items.push("Enter Boss Room"),
        _ => {}
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Keyboard (Esc = Open Menu), Map (S = Shop, B = Boss Room)");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!(
            "Dungeon Floor {}, Room {}",
            dungeon_floor.floor, current_room.room_num
        );
        execute!(stdout, cursor::MoveTo(0, 2))?;

        let start_column = match current_room.kind {
            RoomKind::Start => display_start_room(2)?,
            RoomKind::TwoWayUpDown => display_twowayupdown_room(2)?,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid Dungeon Room",
                ))
            }
        };

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Esc => {
                    if let Ok(return_to_main_menu) = menu_character(&mut character) {
                        if return_to_main_menu {
                            return Ok(GameMenuReturnOptions {
                                main_menu: true,
                                rerender: false,
                            });
                        } else {
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    match menu_items[selected_index] {
        "Go Up" => {}
        "Go Down" => {}
        "Go Right" => {}
        "Go Left" => {}
        _ => {}
    }

    Ok(GameMenuReturnOptions {
        main_menu: false,
        rerender: false,
    })
}
