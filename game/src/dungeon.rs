use crate::{
    character::CharacterClass,
    enemy::{generate_random_boss_enemy, generate_random_normal_enemy, Enemy},
    shop::{generate_shop_items, ShopItems},
};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub mod room;

pub const NORMAL_ENEMIES_PER_FLOOR: u32 = 3;
pub const MIN_ROOMS_FOR_BOSS_ENTRANCE: u32 = 6;
/// Increasing this makes dungeon floors have more rooms.
pub const FLOOR_LENGTH_SCALE: u32 = 9;

pub struct DungeonFloor {
    pub floor: u32,
    pub rooms: HashMap<RoomCoordinates, Room>,
    pub boss: Option<Enemy>,
    pub shop_items: ShopItems,
}

impl DungeonFloor {
    pub fn new(floor: u32, rooms: HashMap<RoomCoordinates, Room>, class: &CharacterClass) -> Self {
        Self {
            floor,
            rooms,
            boss: Some(generate_random_boss_enemy(floor)),
            shop_items: generate_shop_items(floor, class),
        }
    }

    pub fn pretty_print(&self) {
        println!("Dungeon Floor: {}", self.floor);
        println!("Rooms: {}", self.rooms.len());
        for room in self.rooms.values() {
            println!("  Room:");
            println!("    Coordinates: ({}, {})", room.coords.x, room.coords.y);
            println!("    Kind: {:?}", room.kind);
            println!("    Enemy: {:?}", room.enemy);
            println!("    Treasure: {}", room.treasure);
            println!("    Adjacent Rooms: {:?}\n", room.adjacents);
        }
    }
}

/// Room grid coordinates. Start room has coordinates (0,0).
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RoomCoordinates {
    pub x: i32,
    pub y: i32,
}

impl RoomCoordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub kind: RoomKind,
    pub coords: RoomCoordinates,
    pub adjacents: AdjacentRooms,
    pub enemy: Option<Enemy>,
    pub treasure: bool,
    pub room_num: u32,
}

#[derive(Debug, Clone)]
pub struct AdjacentRooms {
    pub up: Option<RoomCoordinates>,
    pub down: Option<RoomCoordinates>,
    pub left: Option<RoomCoordinates>,
    pub right: Option<RoomCoordinates>,
}

impl Room {
    pub fn new(kind: RoomKind, coords: RoomCoordinates, num: u32) -> Self {
        Self {
            coords,
            kind,
            adjacents: AdjacentRooms {
                up: None,
                down: None,
                left: None,
                right: None,
            },
            enemy: None,
            treasure: false,
            room_num: num,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RoomKind {
    /// OneWayUp room, contains shop interaction.
    Start,
    /// Contains door to next floor.
    Boss,
    /// Contains boss room interaction.
    BossEntrance,
    TwoWayUpDown,
    TwoWayLeftRight,
    TwoWayUpLeft,
    TwoWayUpRight,
    TwoWayDownLeft,
    TwoWayDownRight,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

pub fn generate_random_dungeon_floor(floor: u32, class: &CharacterClass) -> DungeonFloor {
    let mut rooms = HashMap::new();
    let start_room = Room::new(RoomKind::Start, RoomCoordinates::new(0, 0), 1);
    rooms.insert(start_room.coords.clone(), start_room.clone());
    generate_random_rooms(start_room, &mut rooms, FLOOR_LENGTH_SCALE);
    randomize_treasure_room(&mut rooms);
    randomize_enemy_rooms(&mut rooms, NORMAL_ENEMIES_PER_FLOOR, floor);
    return DungeonFloor::new(floor, rooms, class);
}

fn connect_rooms(
    room1: &mut Room,
    room2: &mut Room,
    room1_direction: &Direction,
    room2_direction: &Direction,
) {
    match room1_direction {
        Direction::Up => room1.adjacents.up = Some(room2.coords.clone()),
        Direction::Down => room1.adjacents.down = Some(room2.coords.clone()),
        Direction::Left => room1.adjacents.left = Some(room2.coords.clone()),
        Direction::Right => room1.adjacents.right = Some(room2.coords.clone()),
        _ => {}
    }
    match room2_direction {
        Direction::Up => room2.adjacents.up = Some(room1.coords.clone()),
        Direction::Down => room2.adjacents.down = Some(room1.coords.clone()),
        Direction::Left => room2.adjacents.left = Some(room1.coords.clone()),
        Direction::Right => room2.adjacents.right = Some(room1.coords.clone()),
        _ => {}
    }
}

fn generate_random_rooms(
    start_room: Room,
    rooms: &mut HashMap<RoomCoordinates, Room>,
    length_scale: u32,
) {
    let mut rng = thread_rng();
    let mut rooms_generated = 1;
    let mut boss_entrance_generated = false;
    let mut boss_room_generated = false;
    let mut current = start_room;
    let mut current_direction = Direction::Unknown;

    loop {
        if boss_room_generated {
            break;
        }
        let mut room_kind = RoomKind::Unknown;
        let mut room_direction = Direction::Unknown;
        let mut room_coordinates = RoomCoordinates::new(0, 0);

        match current.kind {
            RoomKind::Start => {
                room_kind = RoomKind::TwoWayUpDown;
                current_direction = Direction::Up;
                room_direction = Direction::Down;
                room_coordinates = RoomCoordinates::new(0, 1);
            }
            RoomKind::BossEntrance => {
                room_kind = RoomKind::Boss;
                current_direction = Direction::Up;
                room_direction = Direction::Down;
                room_coordinates = RoomCoordinates::new(current.coords.x, current.coords.y + 1);
            }
            RoomKind::TwoWayUpDown => {
                if rooms_generated > length_scale {
                    room_kind = RoomKind::BossEntrance;
                } else {
                    let mut possible_rooms = vec![
                        RoomKind::TwoWayUpDown,
                        RoomKind::TwoWayDownRight,
                        RoomKind::TwoWayDownLeft,
                    ];
                    if rooms_generated >= MIN_ROOMS_FOR_BOSS_ENTRANCE && !boss_entrance_generated {
                        possible_rooms.push(RoomKind::BossEntrance);
                    }
                    let rand_num = rng.gen_range(0..possible_rooms.len());
                    room_kind = possible_rooms[rand_num].clone();
                }
                current_direction = Direction::Up;
                room_direction = Direction::Down;
                room_coordinates = RoomCoordinates::new(current.coords.x, current.coords.y + 1);
            }
            RoomKind::TwoWayDownRight => {
                let possible_rooms = vec![RoomKind::TwoWayLeftRight, RoomKind::TwoWayUpLeft];
                let rand_num = rng.gen_range(0..possible_rooms.len());
                room_kind = possible_rooms[rand_num].clone();
                current_direction = Direction::Right;
                room_direction = Direction::Left;
                room_coordinates = RoomCoordinates::new(current.coords.x + 1, current.coords.y);
            }
            RoomKind::TwoWayDownLeft => {
                let possible_rooms = vec![RoomKind::TwoWayLeftRight, RoomKind::TwoWayUpRight];
                let rand_num = rng.gen_range(0..possible_rooms.len());
                room_kind = possible_rooms[rand_num].clone();
                current_direction = Direction::Left;
                room_direction = Direction::Right;
                room_coordinates = RoomCoordinates::new(current.coords.x - 1, current.coords.y);
            }
            RoomKind::TwoWayLeftRight => {
                let mut possible_rooms = vec![RoomKind::TwoWayLeftRight];
                match current_direction {
                    Direction::Right => possible_rooms.push(RoomKind::TwoWayUpLeft),
                    Direction::Left => possible_rooms.push(RoomKind::TwoWayUpRight),
                    _ => {}
                }
                let rand_num = rng.gen_range(0..possible_rooms.len());
                room_kind = possible_rooms[rand_num].clone();
                match current_direction {
                    Direction::Right => {
                        room_direction = Direction::Left;
                        room_coordinates =
                            RoomCoordinates::new(current.coords.x + 1, current.coords.y);
                    }
                    Direction::Left => {
                        room_direction = Direction::Right;
                        room_coordinates =
                            RoomCoordinates::new(current.coords.x - 1, current.coords.y);
                    }
                    _ => {}
                }
            }
            RoomKind::TwoWayUpRight => {
                if rooms_generated > length_scale {
                    room_kind = RoomKind::BossEntrance;
                } else {
                    let mut possible_rooms = vec![
                        RoomKind::TwoWayUpDown,
                        RoomKind::TwoWayDownRight,
                        RoomKind::TwoWayDownLeft,
                    ];
                    if rooms_generated >= MIN_ROOMS_FOR_BOSS_ENTRANCE && !boss_entrance_generated {
                        possible_rooms.push(RoomKind::BossEntrance);
                    }
                    let rand_num = rng.gen_range(0..possible_rooms.len());
                    room_kind = possible_rooms[rand_num].clone();
                }
                current_direction = Direction::Up;
                room_direction = Direction::Down;
                room_coordinates = RoomCoordinates::new(current.coords.x, current.coords.y + 1);
            }
            RoomKind::TwoWayUpLeft => {
                if rooms_generated > length_scale {
                    room_kind = RoomKind::BossEntrance;
                } else {
                    let mut possible_rooms = vec![
                        RoomKind::TwoWayUpDown,
                        RoomKind::TwoWayDownRight,
                        RoomKind::TwoWayDownLeft,
                    ];
                    if rooms_generated >= MIN_ROOMS_FOR_BOSS_ENTRANCE && !boss_entrance_generated {
                        possible_rooms.push(RoomKind::BossEntrance);
                    }
                    let rand_num = rng.gen_range(0..possible_rooms.len());
                    room_kind = possible_rooms[rand_num].clone();
                }
                current_direction = Direction::Up;
                room_direction = Direction::Down;
                room_coordinates = RoomCoordinates::new(current.coords.x, current.coords.y + 1);
            }
            _ => {}
        }

        match room_kind {
            RoomKind::BossEntrance => {
                boss_entrance_generated = true;
            }
            RoomKind::Boss => {
                boss_room_generated = true;
            }
            _ => {}
        }
        let mut room = Room::new(room_kind.clone(), room_coordinates, rooms_generated + 1);
        connect_rooms(&mut current, &mut room, &current_direction, &room_direction);
        rooms.insert(room.coords.clone(), room.clone());
        rooms.insert(current.coords.clone(), current);
        rooms_generated += 1;
        current = room;
    }
}

fn randomize_enemy_rooms(
    rooms: &mut HashMap<RoomCoordinates, Room>,
    enemies_per_floor: u32,
    dungeon_floor: u32,
) {
    let mut temp_rooms = Vec::new();
    for room in rooms.values() {
        match room.kind {
            // don't spawn normal enemies in these rooms
            RoomKind::Start | RoomKind::BossEntrance | RoomKind::Boss => continue,
            _ => temp_rooms.push(room.clone()),
        }
    }
    let mut rng = thread_rng();
    for _ in 0..enemies_per_floor {
        loop {
            let rand_num = rng.gen_range(0..temp_rooms.len());
            let rand_room = &temp_rooms[rand_num];
            match rand_room.enemy {
                None => {
                    let enemy = generate_random_normal_enemy(dungeon_floor);
                    if let Some(room) = rooms.get_mut(&rand_room.coords) {
                        room.enemy = Some(enemy.clone());
                        temp_rooms[rand_num].enemy = Some(enemy);
                    }
                    break;
                }
                _ => {}
            }
        }
    }
}

fn randomize_treasure_room(rooms: &mut HashMap<RoomCoordinates, Room>) {
    let mut temp_rooms = Vec::new();
    for room in rooms.values() {
        match room.kind {
            RoomKind::Start | RoomKind::BossEntrance => continue,
            _ => temp_rooms.push(room.clone()),
        }
    }
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(0..temp_rooms.len());
    let treasure_room = temp_rooms[rand_num].clone();
    if let Some(room) = rooms.get_mut(&treasure_room.coords) {
        room.treasure = true;
    }
}
