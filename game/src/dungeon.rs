pub struct DungeonFloor {
    pub floor: u32,
}

impl DungeonFloor {
    pub fn new(floor: u32) -> Self {
        Self { floor }
    }
}

pub fn generate_random_dungeon_floor(floor: u32) -> DungeonFloor {
    return DungeonFloor::new(floor);
}
