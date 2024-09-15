use crate::{
    currency::{random_gold_amount, BASE_GOLD_MAX, BASE_GOLD_MIN, GOLD_MULTIPLIER_TREASURE_CHEST},
    dungeon::{
        generate_random_dungeon_floor,
        room::{
            display_boss_entrance_room, display_start_room, display_two_way_down_left_room,
            display_two_way_down_right_room, display_two_way_left_right_room,
            display_two_way_up_down_room, display_two_way_up_left_room,
            display_two_way_up_right_room,
        },
        DungeonFloor, Room, RoomCoordinates, RoomKind,
    },
    items::{
        generate_random_armor, generate_random_ring, generate_random_weapon, get_item_display_name,
        random_equipment_item, CharacterItem, ItemCategory, ARMOR_BASE_VALUES, RING_BASE_VALUES,
        WEAPON_BASE_VALUES,
    },
    menu::character::menu_character,
    session::{Player, PlayerCharacter},
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
        let character = match &player.character {
            Some(character) => character,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No selected character",
                ))
            }
        };
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
                KeyCode::Enter => match menu_items[selected_index] {
                    "Start Dungeon Floor" => {
                        let mut dungeon_floor = generate_random_dungeon_floor(
                            character.data.stats.general_stats.current_dungeon_floor,
                        );
                        let mut next_room_coords = RoomCoordinates::new(0, 0);
                        loop {
                            let opts =
                                menu_dungeon_floor(&mut dungeon_floor, player, &next_room_coords)?;
                            if opts.return_to_main_menu {
                                return Ok(true);
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
    let mut character = match &mut player.character {
        Some(character) => character,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No selected character",
            ))
        }
    };
    let current_room = match dungeon_floor.rooms.get_mut(current_room_coords) {
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
    if current_room.treasure {
        menu_items.push("Open Treasure Chest");
    }

    loop {
        let mut start_column = 2;
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Keyboard (Esc = Open Menu), Map (S = Shop, B = Boss Room)");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!(
            "Dungeon Floor {}, Room {}",
            dungeon_floor.floor, current_room.room_num
        );
        execute!(stdout, cursor::MoveTo(0, 2))?;

        start_column = match current_room.kind {
            RoomKind::Start => display_start_room(start_column)?,
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
                    "Enter Shop" => {}
                    "Enter Boss Room" => {}
                    "Open Treasure Chest" => {
                        menu_open_treasure_chest(dungeon_floor.floor, character, current_room)?;
                        menu_items.remove(selected_index);
                        selected_index = 0;
                    }
                    _ => break,
                },
                KeyCode::Esc => {
                    if let Ok(return_to_main_menu) = menu_character(&mut character) {
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

    let gold = random_gold_amount(
        BASE_GOLD_MIN,
        BASE_GOLD_MAX,
        GOLD_MULTIPLIER_TREASURE_CHEST,
        dungeon_floor,
    );
    character.give_gold(gold);

    let equipment_item_category = random_equipment_item();
    let mut item_display_name = "?Unknown?".to_string();
    match equipment_item_category {
        ItemCategory::Weapon => {
            let weapon = generate_random_weapon(WEAPON_BASE_VALUES, dungeon_floor);
            character.give_weapon(&weapon);
            item_display_name = get_item_display_name(CharacterItem::Weapon(&weapon));
        }
        ItemCategory::Armor => {
            let armor = generate_random_armor(ARMOR_BASE_VALUES, dungeon_floor);
            character.give_armor(&armor);
            item_display_name = get_item_display_name(CharacterItem::Armor(&armor));
        }
        ItemCategory::Ring => {
            let ring = generate_random_ring(RING_BASE_VALUES, dungeon_floor);
            character.give_ring(&ring);
            item_display_name = get_item_display_name(CharacterItem::Ring(&ring));
        }
        _ => {}
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("You opened a treasure chest");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Drops:");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("  Gold: {}", gold);
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!("  Item: {}", item_display_name);
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
