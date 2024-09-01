use std::cell::RefCell;
use std::rc::Rc;

pub struct DungeonFloor {
    pub floor: u32,
    pub start_room: Rc<RefCell<Room>>,
}

impl DungeonFloor {
    pub fn new(floor: u32, start_room: Rc<RefCell<Room>>) -> Self {
        Self { floor, start_room }
    }
}

pub struct Room {
    pub kind: RoomKind,
    pub adjacent_rooms: AdjacentRooms,
}

pub struct AdjacentRooms {
    pub up: Option<Rc<RefCell<Room>>>,
    pub down: Option<Rc<RefCell<Room>>>,
    pub left: Option<Rc<RefCell<Room>>>,
    pub right: Option<Rc<RefCell<Room>>>,
}

impl Room {
    pub fn new(kind: RoomKind) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Room {
            kind,
            adjacent_rooms: AdjacentRooms {
                up: None,
                down: None,
                left: None,
                right: None,
            },
        }))
    }
}

pub enum RoomKind {
    Start,
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
    TwoWayLeftDown,
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

pub fn generate_random_dungeon_floor(floor: u32) -> DungeonFloor {
    let start_room = Room::new(RoomKind::Start);
    return DungeonFloor::new(floor, start_room);
}
