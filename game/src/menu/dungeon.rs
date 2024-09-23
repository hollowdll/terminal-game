use crate::{
    drops::give_treasure_chest_drops,
    dungeon::{
        generate_random_dungeon_floor,
        room::{
            display_boss_entrance_room, display_boss_room, display_start_room,
            display_two_way_down_left_room, display_two_way_down_right_room,
            display_two_way_left_right_room, display_two_way_up_down_room,
            display_two_way_up_left_room, display_two_way_up_right_room,
        },
        DungeonFloor, Room, RoomCoordinates, RoomKind,
    },
    game::save_game,
    menu::{character::menu_character, shop::menu_shop},
    session::{Player, PlayerCharacter},
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use super::enemy::menu_enemy_encounter;

pub struct GameMenuReturnOptions {
    /// If should return to main menu.
    pub main_menu: bool,
    /// If should rerender current menu.
    pub rerender: bool,
}

pub struct DungeonFloorMenuOptions {
    pub return_to_main_menu: bool,
    pub dungeon_completed: bool,
    pub game_over: bool,
    pub next_room_coords: Option<RoomCoordinates>,
}

/// Returns true if should go back to main menu.
pub fn menu_start_dungeon_floor(player: &mut Player) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Start Dungeon Floor", "Return to main menu"];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    loop {
        let character = player.get_character()?;
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!(
            "Character: {} (Level {} {:?}, Dungeon Floor {})",
            character.data.metadata.name,
            character.data.stats.general_stats.character_level,
            character.data.metadata.class,
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
                KeyCode::Enter => match menu_items[selected_index] {
                    "Start Dungeon Floor" => {
                        let mut dungeon_floor = generate_random_dungeon_floor(
                            character.data.stats.general_stats.current_dungeon_floor,
                            &character.data.metadata.class,
                        );
                        let mut next_room_coords = RoomCoordinates::new(0, 0);
                        loop {
                            let opts =
                                menu_dungeon_floor(&mut dungeon_floor, player, &next_room_coords)?;
                            if opts.return_to_main_menu {
                                return Ok(true);
                            }
                            if opts.game_over {
                                player.get_character_mut()?.reset_character_on_death();
                                save_game(player)?;
                                execute!(stdout, Clear(ClearType::All))?;
                                break;
                            }
                            if opts.dungeon_completed {
                                save_game(player)?;
                                execute!(stdout, Clear(ClearType::All))?;
                                break;
                            }
                            if let Some(coords) = opts.next_room_coords {
                                next_room_coords = coords;
                            }
                        }
                    }
                    "Return to main menu" => break,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(true)
}

/// Returns true if should go back to main menu.
pub fn menu_dungeon_floor(
    dungeon_floor: &mut DungeonFloor,
    player: &mut Player,
    current_room_coords: &RoomCoordinates,
) -> io::Result<DungeonFloorMenuOptions> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    let mut menu_items = Vec::new();
    let mut selected_index = 0;

    let current_room = match dungeon_floor.rooms.get_mut(current_room_coords) {
        Some(room) => room,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Dungeon Room should exist, but was not found",
            ))
        }
    };

    if let Some(enemy) = &mut current_room.enemy {
        let victory = menu_enemy_encounter(enemy, player.get_character_mut()?)?;
        if victory {
            current_room.enemy = None;
        } else {
            return Ok(DungeonFloorMenuOptions {
                return_to_main_menu: false,
                dungeon_completed: false,
                game_over: true,
                next_room_coords: None,
            });
        }
    }

    if let Some(_) = current_room.adjacents.up {
        match current_room.kind {
            RoomKind::BossEntrance => {}
            _ => menu_items.push("Go Up"),
        }
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
        RoomKind::Boss => {
            menu_items.push("Enter Next Floor");
            if let Some(boss) = &mut dungeon_floor.boss {
                let victory = menu_enemy_encounter(boss, player.get_character_mut()?)?;
                if victory {
                    dungeon_floor.boss = None;
                    let character = player.get_character_mut()?;
                    character.dungeon_floor_completed(dungeon_floor.floor + 1);
                    save_game(player)?;
                } else {
                    return Ok(DungeonFloorMenuOptions {
                        return_to_main_menu: false,
                        dungeon_completed: false,
                        game_over: true,
                        next_room_coords: None,
                    });
                }
            }
        }
        _ => {}
    }
    if current_room.treasure {
        menu_items.push("Open Treasure Chest");
    }

    loop {
        let mut start_column = 2;
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Keyboard (Esc = Open Menu), Map (S = Shop, B = Boss Room)");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        match current_room.kind {
            RoomKind::Boss => println!("Dungeon Floor {}, Boss Room", dungeon_floor.floor),
            _ => println!(
                "Dungeon Floor {}, Room {}",
                dungeon_floor.floor, current_room.room_num
            ),
        }
        execute!(stdout, cursor::MoveTo(0, 2))?;

        start_column = match current_room.kind {
            RoomKind::Start => display_start_room(start_column)?,
            RoomKind::Boss => display_boss_room(start_column, dungeon_floor.floor + 1)?,
            RoomKind::BossEntrance => display_boss_entrance_room(start_column)?,
            RoomKind::TwoWayUpDown => display_two_way_up_down_room(start_column)?,
            RoomKind::TwoWayLeftRight => display_two_way_left_right_room(start_column)?,
            RoomKind::TwoWayUpLeft => display_two_way_up_left_room(start_column)?,
            RoomKind::TwoWayUpRight => display_two_way_up_right_room(start_column)?,
            RoomKind::TwoWayDownLeft => display_two_way_down_left_room(start_column)?,
            RoomKind::TwoWayDownRight => display_two_way_down_right_room(start_column)?,
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
                KeyCode::Enter => match menu_items[selected_index] {
                    "Go Up" => {
                        return Ok(DungeonFloorMenuOptions {
                            return_to_main_menu: false,
                            dungeon_completed: false,
                            game_over: false,
                            next_room_coords: current_room.adjacents.up.clone(),
                        })
                    }
                    "Go Down" => {
                        return Ok(DungeonFloorMenuOptions {
                            return_to_main_menu: false,
                            dungeon_completed: false,
                            game_over: false,
                            next_room_coords: current_room.adjacents.down.clone(),
                        })
                    }
                    "Go Right" => {
                        return Ok(DungeonFloorMenuOptions {
                            return_to_main_menu: false,
                            dungeon_completed: false,
                            game_over: false,
                            next_room_coords: current_room.adjacents.right.clone(),
                        })
                    }
                    "Go Left" => {
                        return Ok(DungeonFloorMenuOptions {
                            return_to_main_menu: false,
                            dungeon_completed: false,
                            game_over: false,
                            next_room_coords: current_room.adjacents.left.clone(),
                        })
                    }
                    "Enter Shop" => {
                        menu_shop(&mut dungeon_floor.shop_items, player.get_character_mut()?)?;
                    }
                    "Enter Boss Room" => {
                        return Ok(DungeonFloorMenuOptions {
                            return_to_main_menu: false,
                            dungeon_completed: false,
                            game_over: false,
                            next_room_coords: current_room.adjacents.up.clone(),
                        })
                    }
                    "Enter Next Floor" => {
                        return Ok(DungeonFloorMenuOptions {
                            return_to_main_menu: false,
                            dungeon_completed: true,
                            game_over: false,
                            next_room_coords: None,
                        })
                    }
                    "Open Treasure Chest" => {
                        menu_open_treasure_chest(
                            dungeon_floor.floor,
                            player.get_character_mut()?,
                            current_room,
                        )?;
                        menu_items.remove(selected_index);
                        selected_index = 0;
                    }
                    _ => break,
                },
                KeyCode::Esc => {
                    if let Ok(return_to_main_menu) = menu_character(player.get_character_mut()?) {
                        if return_to_main_menu {
                            return Ok(DungeonFloorMenuOptions {
                                return_to_main_menu: true,
                                dungeon_completed: false,
                                game_over: false,
                                next_room_coords: None,
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

    Ok(DungeonFloorMenuOptions {
        return_to_main_menu: true,
        dungeon_completed: false,
        game_over: false,
        next_room_coords: None,
    })
}

pub fn menu_open_treasure_chest(
    dungeon_floor: u32,
    character: &mut PlayerCharacter,
    current_room: &mut Room,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    let drops = give_treasure_chest_drops(character, dungeon_floor);

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("You opened a treasure chest");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Drops:");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("  Gold: {}", drops.gold);
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!("  Item: {}", drops.equipment_item_name);
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!("> Continue");

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }
    current_room.treasure = false;
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}
