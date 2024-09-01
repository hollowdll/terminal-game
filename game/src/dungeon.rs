pub struct DungeonFloor {
    pub floor: u32,
}

impl DungeonFloor {
    pub fn new(floor: u32) -> Self {
        Self { floor }
    }
}

pub struct DungeonRoom {
    pub kind: DungeonRoomKind,
}

pub enum DungeonRoomKind {
    Start,
    Boss,
    OneWay,
    TwoWay,
    ThreeWay,
    FourWay,
}

pub fn generate_random_dungeon_floor(floor: u32) -> DungeonFloor {
    return DungeonFloor::new(floor);
}
