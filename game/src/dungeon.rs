use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub const MAX_ROOMS_PER_PATH: u32 = 3;
pub const ENEMY_SPAWN_CHANCE_PER_ROOM: f64 = 0.5;
pub const MIN_ENEMIES_PER_FLOOR: u32 = 1;

pub const START_ROOM_POSSIBLE_ADJACENTS: [RoomKind; 7] = [
    RoomKind::TwoWayUpDown,
    RoomKind::TwoWayDownLeft,
    RoomKind::TwoWayDownRight,
    RoomKind::ThreeWayDownLeftRight,
    RoomKind::ThreeWayUpDownLeft,
    RoomKind::ThreeWayUpDownRight,
    RoomKind::FourWay,
];

#[derive(Debug)]
pub struct DungeonFloor {
    pub floor: u32,
    pub start_room: Room,
    pub rooms: HashMap<RoomCoordinates, Room>,
}

impl DungeonFloor {
    pub fn new(floor: u32, start_room: Room, rooms: HashMap<RoomCoordinates, Room>) -> Self {
        Self {
            floor,
            start_room,
            rooms,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RoomCoordinates {
    pub x: i32,
    pub y: i32,
}

impl RoomCoordinates {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub kind: RoomKind,
    pub coords: RoomCoordinates,
    pub adjacents: AdjacentRooms,
}

#[derive(Debug, Clone)]
pub struct AdjacentRooms {
    pub up: Option<RoomCoordinates>,
    pub down: Option<RoomCoordinates>,
    pub left: Option<RoomCoordinates>,
    pub right: Option<RoomCoordinates>,
}

impl Room {
    pub fn new(kind: RoomKind, coords: RoomCoordinates) -> Self {
        Self {
            coords,
            kind,
            adjacents: AdjacentRooms {
                up: None,
                down: None,
                left: None,
                right: None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum RoomKind {
    /// OneWayUp room, contains shop interaction.
    Start,
    /// OneWayDown room, contains boss room interaction.
    BossEntrance,
    OneWayUp,
    OneWayDown,
    OneWayLeft,
    OneWayRight,
    TreasureUp,
    TreasureDown,
    TreasureLeft,
    TreasureRight,
    TwoWayUpDown,
    TwoWayLeftRight,
    TwoWayUpLeft,
    TwoWayUpRight,
    TwoWayDownLeft,
    TwoWayDownRight,
    ThreeWayDownLeftRight,
    ThreeWayUpLeftRight,
    ThreeWayUpDownLeft,
    ThreeWayUpDownRight,
    FourWay,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn generate_random_dungeon_floor(floor: u32) -> DungeonFloor {
    let mut rooms = HashMap::new();
    let mut start_room = Room::new(RoomKind::Start, RoomCoordinates::new(0, 0));
    rooms.insert(start_room.coords.clone(), start_room.clone());
    generate_start_room_adjacent(&mut start_room, &mut rooms);

    DungeonFloor::new(floor, start_room, rooms)
}

fn generate_start_room_adjacent(start_room: &mut Room, rooms: &mut HashMap<RoomCoordinates, Room>) {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(0..START_ROOM_POSSIBLE_ADJACENTS.len());
    let rand_room_kind = &START_ROOM_POSSIBLE_ADJACENTS[rand_num];
    let mut room = Room::new(rand_room_kind.clone(), RoomCoordinates::new(0, 1));

    start_room.adjacents.up = Some(room.coords.clone());
    room.adjacents.down = Some(start_room.coords.clone());
    rooms.insert(room.coords.clone(), room);
}
