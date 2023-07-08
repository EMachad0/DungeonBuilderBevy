use crate::dungeon::room::Room;

pub struct DungeonLevel {
    pub rooms: Vec<Room>,
}

impl DungeonLevel {
    pub fn level_1() -> Self {
        let rooms = vec![
            Room::entrance_bottom(),
            Room::corridor_horizontal(),
            Room::corridor_vertical(),
            Room::small_room(),
        ];

        Self { rooms }
    }
}
