use std::io;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};

use crate::{
    dungeon::{
        room::{display_start_room, display_twowayupdown_room},
        DungeonFloor, RoomCoordinates, RoomKind,
    },
    game_data::write_save_file,
    items::ItemCategory,
    session::{Player, PlayerCharacter},
    util::timestamp_to_datetime,
};

pub struct GameMenuReturnOptions {
    /// If should return to main menu.
    pub main_menu: bool,
    /// If should rerender current menu.
    pub rerender: bool,
}

pub fn save_game(player: &mut Player) -> io::Result<()> {
    if let Some(player_character) = &player.character {
        player.data.characters.insert(
            player_character.data.metadata.name.clone(),
            player_character.data.clone(),
        );
    }
    write_save_file(&player.data)?;
    Ok(())
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

/// Returns true if should go back to main menu.
pub fn menu_character(character: &mut PlayerCharacter) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec![
        "Stats",
        "Inventory",
        "Equipment",
        "Return to main menu",
        "Back",
    ];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Menu");
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
                KeyCode::Enter => match menu_items[selected_index] {
                    "Stats" => {
                        menu_character_stats(&character)?;
                    }
                    "Inventory" => {
                        menu_inventory(character)?;
                    }
                    "Equipment" => {}
                    "Return to main menu" => return Ok(true),
                    _ => break,
                },
                _ => {}
            }
        }
    }

    Ok(false)
}

fn menu_character_stats(character: &PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Back"];
    let mut selected_index = 0;
    let start_column: u16 = 22;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Character Stats");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("  General:");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("    Name: {}", character.data.metadata.name);
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!(
            "    Created At: {}",
            timestamp_to_datetime(character.data.metadata.created_at)
        );
        execute!(stdout, cursor::MoveTo(0, 4))?;
        println!(
            "    Current Dungeon Floor: {}",
            character.data.stats.general_stats.current_dungeon_floor
        );
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!(
            "    Level: {}",
            character.data.stats.general_stats.character_level
        );
        execute!(stdout, cursor::MoveTo(0, 6))?;
        println!(
            "    EXP: {}",
            character.data.stats.general_stats.current_exp
        );
        execute!(stdout, cursor::MoveTo(0, 7))?;
        println!(
            "    Required EXP: {}",
            character.data.stats.general_stats.required_exp
        );
        execute!(stdout, cursor::MoveTo(0, 8))?;
        println!(
            "    Total EXP: {}",
            character.data.stats.general_stats.total_exp
        );
        execute!(stdout, cursor::MoveTo(0, 9))?;

        println!(
            "    Highest Dungeon Floor Reached: {}",
            character
                .data
                .stats
                .general_stats
                .highest_dungeon_floor_achieved
        );
        execute!(stdout, cursor::MoveTo(0, 10))?;
        println!(
            "    Highest Level Reached: {}",
            character
                .data
                .stats
                .general_stats
                .highest_character_level_achieved
        );
        execute!(stdout, cursor::MoveTo(0, 11))?;
        println!(
            "    Total Deaths: {}",
            character.data.stats.general_stats.deaths
        );
        execute!(stdout, cursor::MoveTo(0, 12))?;

        println!("  Combat:");
        execute!(stdout, cursor::MoveTo(0, 13))?;
        println!(
            "    Max Health: {}",
            character.data.stats.combat_stats.max_health
        );
        execute!(stdout, cursor::MoveTo(0, 14))?;
        println!("    Health: {}", character.temp_stats.health);
        execute!(stdout, cursor::MoveTo(0, 15))?;
        println!(
            "    Max Mana: {}",
            character.data.stats.combat_stats.max_mana
        );
        execute!(stdout, cursor::MoveTo(0, 16))?;
        println!("    Mana: {}", character.temp_stats.mana);
        execute!(stdout, cursor::MoveTo(0, 17))?;
        println!("    Defense: {}", character.data.stats.combat_stats.defense);
        execute!(stdout, cursor::MoveTo(0, 18))?;
        println!("    Damage: {}", character.data.stats.combat_stats.damage);
        execute!(stdout, cursor::MoveTo(0, 19))?;
        println!(
            "    Critical Damage Multiplier: {}",
            character.data.stats.combat_stats.critical_damage_multiplier
        );
        execute!(stdout, cursor::MoveTo(0, 20))?;
        println!(
            "    Critical Hit Rate: {}",
            character.data.stats.combat_stats.critical_hit_rate
        );
        execute!(stdout, cursor::MoveTo(0, 21))?;

        println!("");
        execute!(stdout, cursor::MoveTo(0, 22))?;

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
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

fn menu_inventory(character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Consumables", "Weapons", "Armors", "Rings", "Back"];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Inventory");
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
        "Consumables" => {}
        "Weapons" => {}
        "Armors" => {}
        "Rings" => {}
        _ => {}
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

fn menu_inventory_item_list(
    character: &mut PlayerCharacter,
    item_category: ItemCategory,
) -> io::Result<()> {
    Ok(())
}
