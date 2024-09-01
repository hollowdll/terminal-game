use crate::session::PlayerCharacter;

pub const TREASURE_CHEST_DESC: &str =
    "Open this chest to get gold and a random rarity weapon, armor, or ring.";

pub struct TreasureChest {
    pub gold: u64,
    pub description: &'static str,
}

impl TreasureChest {
    pub fn new(dungeon_floor: u32) -> Self {
        Self {
            gold: 100 + (25 * dungeon_floor as u64),
            description: TREASURE_CHEST_DESC,
        }
    }

    pub fn open(&self, player_char: &mut PlayerCharacter) {
        player_char.data.currency.gold += self.gold;
        // TODO item drop
    }
}
