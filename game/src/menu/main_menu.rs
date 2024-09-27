use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, Write};

use crate::{
    character::{
        create_new_game_character, delete_game_character, get_character_skill,
        get_character_skill_description, get_class_starting_stats, load_game_character,
        max_game_characters_reached, CharacterClass,
    },
    config::GameConfig,
    game::save_game,
    menu::dungeon::menu_start_dungeon_floor,
    session::Player,
    util::extract_first_word,
    validation::{character_name_already_exists, character_name_empty, character_name_too_long},
};

fn print_ascii_title(mut out: &io::Stdout) -> io::Result<()> {
    println!("||||||||  ||||||  |||||    |||     |||  ||  |||   ||   ||||||   ||           |||||    ||||||    ||||||  ");
    execute!(out, cursor::MoveTo(0, 1))?;
    println!("   ||     ||      ||   ||  ||||| |||||  ||  ||||  ||  ||    ||  ||           ||   ||  ||   ||  ||       ");
    execute!(out, cursor::MoveTo(0, 2))?;
    println!("   ||     ||||||  |||||    ||  |||  ||  ||  || || ||  ||||||||  ||           |||||    ||||||   ||  |||  ");
    execute!(out, cursor::MoveTo(0, 3))?;
    println!("   ||     ||      ||   ||  ||       ||  ||  ||  ||||  ||    ||  ||           ||   ||  ||       ||    || ");
    execute!(out, cursor::MoveTo(0, 4))?;
    println!("   ||     ||||||  ||   ||  ||       ||  ||  ||   |||  ||    ||  ||||||       ||   ||  ||        ||||||\n");
    execute!(out, cursor::MoveTo(0, 6))?;

    Ok(())
}

/// Returned bool is true if the menu should be rerendered.
pub fn main_menu(player: &mut Player, cfg: &GameConfig) -> io::Result<bool> {
    let mut stdout = io::stdout();
    let menu_items = vec!["Load Game", "New Game", "Credits", "Quit Game"];
    let mut selected_index = 0;
    let mut start_column: u16 = 6;
    let mut rerender = false;
    let version = env!("CARGO_PKG_VERSION");

    execute!(stdout, Clear(ClearType::All))?;
    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        let _ = print_ascii_title(&stdout);

        if cfg.dev_mode {
            println!("Running in development mode");
            execute!(stdout, cursor::MoveTo(0, 7))?;
            start_column = 7;
        }

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        execute!(stdout, cursor::MoveTo(0, start_column + 5))?;
        println!("v{}", version);

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
    }

    match menu_items[selected_index] {
        "Load Game" => {
            if let Ok(go_back) = menu_load_game(player) {
                if go_back {
                    rerender = true;
                } else {
                    if let Ok(go_back) = menu_start_dungeon_floor(player) {
                        if go_back {
                            rerender = true;
                        }
                    }
                }
            }
        }
        "New Game" => {
            if let Ok(go_back) = menu_new_game(player, cfg) {
                if go_back {
                    rerender = true;
                } else {
                    if let Ok(go_back) = menu_start_dungeon_floor(player) {
                        if go_back {
                            rerender = true;
                        }
                    }
                }
            }
        }
        "Credits" => {
            menu_credits()?;
            rerender = true;
        }
        _ => {}
    }

    Ok(rerender)
}

fn menu_credits() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("(Esc) Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Credits");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("©2024 Juuso Hakala");
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!("Source Code: https://github.com/hollowdll/terminal-game");

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

/// Returns true if should go back in menu.
fn menu_load_game(player: &mut Player) -> io::Result<bool> {
    let mut stdout = io::stdout();
    let mut menu_items = Vec::new();
    let mut selected_index = 0;
    let start_column: u16 = 2;

    for (key, val) in &player.data.characters {
        menu_items.push(format!(
            "{} (Level {} {:?}, Dungeon Floor {})",
            key,
            val.stats.general_stats.character_level,
            &val.metadata.class,
            val.stats.general_stats.current_dungeon_floor,
        ))
    }

    execute!(stdout, Clear(ClearType::All))?;
    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        if player.data.characters.is_empty() {
            println!("(Esc) Back");
            execute!(stdout, cursor::MoveTo(0, 1))?;
            println!("No characters found");
            execute!(stdout, cursor::MoveTo(0, 2))?;
        } else {
            println!("(Esc) = Back, (D) = Delete character");
            execute!(stdout, cursor::MoveTo(0, 1))?;
            println!("Select a character");
            execute!(stdout, cursor::MoveTo(0, 2))?;
        }

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Up => {
                        if !menu_items.is_empty() && selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if !menu_items.is_empty() && selected_index < menu_items.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        if !menu_items.is_empty() {
                            let character_name =
                                extract_first_word(menu_items[selected_index].as_str());
                            load_game_character(character_name, player);
                            return Ok(false);
                        }
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        if !menu_items.is_empty() {
                            let name = extract_first_word(menu_items[selected_index].as_str());
                            let deleted = menu_confirm_character_deletion(player, name)?;
                            if deleted {
                                menu_items.remove(selected_index);
                                selected_index = 0;
                            }
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(true)
}

fn menu_confirm_character_deletion(player: &mut Player, character_name: &str) -> io::Result<bool> {
    let mut stdout = io::stdout();
    let menu_items = vec!["No", "Yes"];
    let mut selected_index = 0;
    let start_column: u16 = 1;
    let mut character_deleted = false;

    execute!(stdout, Clear(ClearType::All))?;
    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!(
            "Delete character {}? It cannot be restored once deleted.",
            character_name
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

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
    }

    match menu_items[selected_index] {
        "Yes" => {
            delete_game_character(character_name, player);
            save_game(player)?;
            character_deleted = true;
        }
        "No" => {}
        _ => {}
    }

    Ok(character_deleted)
}

/// Returns true if menu option "Back" was selected.
fn menu_new_game(player: &mut Player, cfg: &GameConfig) -> io::Result<bool> {
    let mut stdout = io::stdout();
    let menu_items = vec!["Back"];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    execute!(stdout, Clear(ClearType::All))?;
    if !max_game_characters_reached(player) {
        match menu_create_character(player, cfg) {
            Ok(character_created) => {
                if character_created {
                    menu_tutorial()?;
                    return Ok(false);
                } else {
                    return Ok(true);
                }
            }
            Err(e) => return Err(e),
        }
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Maximum characters reached. Delete a character to create a new one.");
        execute!(stdout, cursor::MoveTo(0, 1))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
    }

    match menu_items[selected_index] {
        "Back" => {}
        _ => {}
    }

    Ok(true)
}

pub fn menu_create_character(player: &mut Player, cfg: &GameConfig) -> io::Result<bool> {
    let mut stdout = io::stdout();
    let menu_items = vec!["Yes", "No"];
    let mut selected_index = 0;
    let start_column: u16 = 4;
    let mut enter_name_column: u16 = 2;
    let mut character_created = false;
    let mut msg = "";
    let mut name = String::new();

    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All), Show)?;
    println!("Create a new character");
    execute!(stdout, cursor::MoveTo(0, 1))?;
    disable_raw_mode()?;

    loop {
        let mut is_invalid_name = false;
        print!("Enter name: ");
        stdout.flush()?;
        io::stdin().read_line(&mut name)?;
        name = name.trim().to_string();

        if character_name_empty(&name) {
            msg = "Name cannot be blank";
            is_invalid_name = true;
        } else if character_name_too_long(&name) {
            msg = "Name is too long";
            is_invalid_name = true;
        } else if character_name_already_exists(player, &name) {
            msg = "Character with the name already exists";
            is_invalid_name = true;
        }

        if is_invalid_name {
            execute!(stdout, cursor::MoveTo(0, enter_name_column))?;
            println!("{}", msg);
            execute!(stdout, cursor::MoveTo(0, enter_name_column + 1))?;
            enter_name_column += 2;
            name = "".to_string();
        } else {
            break;
        }
    }
    enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), Hide)?;
    let selected_class = menu_choose_character_class()?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Create the following character?");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Name: {}", name);
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("Class: {:?}", selected_class);

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
    }

    match menu_items[selected_index] {
        "Yes" => {
            create_new_game_character(&name, selected_class, player, cfg);
            save_game(player)?;
            character_created = true;
        }
        "No" => {}
        _ => {}
    }

    Ok(character_created)
}

pub fn menu_choose_character_class() -> io::Result<CharacterClass> {
    let mut stdout = io::stdout();
    let menu_items = vec!["Mage", "Cleric", "Assassin", "Warrior", "Knight"];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    execute!(stdout, Clear(ClearType::All))?;
    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Choose your class");

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }
        execute!(
            stdout,
            cursor::MoveTo(0, start_column + 6),
            Clear(ClearType::FromCursorDown)
        )?;
        let selected_class = match menu_items[selected_index] {
            "Mage" => CharacterClass::Mage,
            "Cleric" => CharacterClass::Cleric,
            "Assassin" => CharacterClass::Assassin,
            "Warrior" => CharacterClass::Warrior,
            "Knight" => CharacterClass::Knight,
            _ => CharacterClass::Mage,
        };
        let skill = get_character_skill(&selected_class);
        let starting_stats = get_class_starting_stats(&selected_class);

        println!("Class: {:?}", selected_class);
        execute!(stdout, cursor::MoveTo(0, start_column + 7))?;
        println!("Skill: {}", &skill);
        execute!(stdout, cursor::MoveTo(0, start_column + 8))?;
        println!(
            "Skill Description: {}",
            get_character_skill_description(&skill)
        );
        execute!(stdout, cursor::MoveTo(0, start_column + 9))?;
        println!("Starting Stats:");
        execute!(stdout, cursor::MoveTo(0, start_column + 10))?;
        println!("  Health: {}", starting_stats.max_health);
        execute!(stdout, cursor::MoveTo(0, start_column + 11))?;
        println!("  Mana: {}", starting_stats.max_mana);
        execute!(stdout, cursor::MoveTo(0, start_column + 12))?;
        println!("  Defense: {}", starting_stats.defense);
        execute!(stdout, cursor::MoveTo(0, start_column + 13))?;
        println!("  Damage: {}", starting_stats.damage);
        execute!(stdout, cursor::MoveTo(0, start_column + 14))?;
        println!(
            "  Critical Damage Multiplier: {:.2}",
            starting_stats.critical_damage_multiplier
        );
        execute!(stdout, cursor::MoveTo(0, start_column + 15))?;
        println!(
            "  Critical Hit Rate: {:.2} ({:.2}%)",
            starting_stats.critical_hit_rate,
            starting_stats.critical_hit_rate * 100.0
        );

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
                        execute!(stdout, Clear(ClearType::All))?;
                        return Ok(selected_class);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn menu_tutorial() -> io::Result<()> {
    let mut stdout = io::stdout();
    let menu_items = vec!["Continue", "Skip Tutorial"];
    let mut selected_index = 0;
    let mut page: u8 = 1;

    execute!(stdout, Clear(ClearType::All))?;
    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("[Tutorial]");
        execute!(stdout, cursor::MoveTo(0, 1))?;

        let start_column: u16 = match page {
            1 => {
                println!("About the game");
                execute!(stdout, cursor::MoveTo(0, 3))?;
                println!("This game is a fantasy RPG game that you play in your terminal.");
                execute!(stdout, cursor::MoveTo(0, 4))?;
                println!("The goal is to build your character as strong as possible");
                execute!(stdout, cursor::MoveTo(0, 5))?;
                println!("and get as far in the dungeon as you can.");
                7
            }
            2 => {
                println!("How to play");
                execute!(stdout, cursor::MoveTo(0, 3))?;
                println!("You play with your keyboard. No mouse required.");
                execute!(stdout, cursor::MoveTo(0, 4))?;
                println!("In the game you travel in dungeon floors defeating enemies.");
                execute!(stdout, cursor::MoveTo(0, 5))?;
                println!("Enemy fights are turn based.");
                execute!(stdout, cursor::MoveTo(0, 6))?;
                println!("Enemies drop items, gold and EXP.");
                execute!(stdout, cursor::MoveTo(0, 7))?;
                println!(
                    "Defeating the boss enemy of the floor allows you to enter the next floor."
                );
                execute!(stdout, cursor::MoveTo(0, 8))?;
                println!(
                    "Your progress will be saved when you complete the current dungeon floor."
                );
                10
            }
            3 => {
                println!("Characters");
                execute!(stdout, cursor::MoveTo(0, 3))?;
                println!("You can create different characters with different classes.");
                execute!(stdout, cursor::MoveTo(0, 4))?;
                println!("Each class has different starting stats and grows differently.");
                execute!(stdout, cursor::MoveTo(0, 5))?;
                println!("Each class also has a unique skill that you can use in enemy fights.");
                execute!(stdout, cursor::MoveTo(0, 6))?;
                println!(
                    "Characters level up when enough EXP is gained. Leveling up increases stats."
                );
                execute!(stdout, cursor::MoveTo(0, 7))?;
                println!("You can have 5 characters active at a time.");
                9
            }
            4 => {
                println!("Dungeons");
                execute!(stdout, cursor::MoveTo(0, 3))?;
                println!("Dungeon floors consist of different rooms.");
                execute!(stdout, cursor::MoveTo(0, 4))?;
                println!("Floors and the enemies in them are randomly generated.");
                execute!(stdout, cursor::MoveTo(0, 5))?;
                println!("Enemies get stronger on each floor.");
                7
            }
            5 => {
                println!("Items");
                execute!(stdout, cursor::MoveTo(0, 3))?;
                println!("There are equipment and consumable items.");
                execute!(stdout, cursor::MoveTo(0, 4))?;
                println!("Equipment items make your character stronger when equipped.");
                execute!(stdout, cursor::MoveTo(0, 5))?;
                println!("Consumables can be used in enemy fights.");
                execute!(stdout, cursor::MoveTo(0, 6))?;
                println!("Items have different rarities.");
                execute!(stdout, cursor::MoveTo(0, 7))?;
                println!("Item rarities are Common, Uncommon, Rare, Epic, Legendary and Mythical.");
                execute!(stdout, cursor::MoveTo(0, 8))?;
                println!("Items can be dropped from enemies or bought in the shop.");
                execute!(stdout, cursor::MoveTo(0, 9))?;
                println!("Mythical items are the strongest items");
                execute!(stdout, cursor::MoveTo(0, 10))?;
                println!("and can only be dropped from the boss of Ancient Ruins.");
                12
            }
            6 => {
                println!("Ancient Ruins");
                execute!(stdout, cursor::MoveTo(0, 3))?;
                println!("Ancient ruins is a special type of dungeon.");
                execute!(stdout, cursor::MoveTo(0, 4))?;
                println!("You need an Ancient Ruins Key to enter it.");
                execute!(stdout, cursor::MoveTo(0, 5))?;
                println!("Ancient Ruins Key has a chance to drop from dungeon bosses.");
                execute!(stdout, cursor::MoveTo(0, 6))?;
                println!("Ancient Ruins has a much stronger boss that you need to fight.");
                execute!(stdout, cursor::MoveTo(0, 7))?;
                println!(
                    "The boss has better rewards than normal bosses and can drop mythical items."
                );
                9
            }
            7 => {
                println!("Dying");
                execute!(stdout, cursor::MoveTo(0, 3))?;
                println!("This game is permadeath.");
                execute!(stdout, cursor::MoveTo(0, 4))?;
                println!("When you die, your character's progress resets");
                execute!(stdout, cursor::MoveTo(0, 5))?;
                println!("and you need to start from dungeon floor 1.");
                execute!(stdout, cursor::MoveTo(0, 6))?;
                println!("Your character's items also reset on death.");
                execute!(stdout, cursor::MoveTo(0, 8))?;
                println!("This game can be challenging and requires some strategy at some points.");
                execute!(stdout, cursor::MoveTo(0, 9))?;
                println!("Good luck and have fun!");
                11
            }
            _ => 1,
        };

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
                        "Skip Tutorial" => break,
                        "Continue" => {
                            if page == 7 {
                                break;
                            }
                            page += 1;
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
