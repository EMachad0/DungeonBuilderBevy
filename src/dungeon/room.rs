use bevy::prelude::{Component, IVec2};

use crate::dungeon::door::Door;
use crate::orientation::Orientation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoomType {
    Entrance,
    SmallRoom,
    MediumRoom,
    LargeRoom,
    CorridorHorizontal,
    CorridorVertical,
}

#[derive(Debug, Clone, Component)]
pub struct Room {
    pub size: IVec2,
    pub room_type: RoomType,
    pub tilemap_path: Box<str>,
    pub doors: Vec<Door>,
}

impl Room {
    pub fn small_room() -> Self {
        Self {
            size: (20, 20).into(),
            room_type: RoomType::SmallRoom,
            tilemap_path: "tiled/tilemaps/room_small.tmx".into(),
            doors: vec![
                Door::new((9, 19), Orientation::North),
                Door::new((18, 9), Orientation::East),
                Door::new((9, 0), Orientation::South),
                Door::new((0, 9), Orientation::West),
            ],
        }
    }

    pub fn entrance_bottom() -> Self {
        Self {
            size: (20, 20).into(),
            room_type: RoomType::Entrance,
            tilemap_path: "tiled/tilemaps/entrance_bottom.tmx".into(),
            doors: vec![Door::new((10, 0), Orientation::South)],
        }
    }

    pub fn corridor_horizontal() -> Self {
        Self {
            size: (10, 10).into(),
            room_type: RoomType::CorridorHorizontal,
            tilemap_path: "tiled/tilemaps/corridor_horizontal_small.tmx".into(),
            doors: vec![
                Door::new((0, 2), Orientation::West),
                Door::new((9, 2), Orientation::East),
            ],
        }
    }

    pub fn corridor_vertical() -> Self {
        Self {
            size: (10, 10).into(),
            room_type: RoomType::CorridorVertical,
            tilemap_path: "tiled/tilemaps/corridor_vertical_small.tmx".into(),
            doors: vec![
                Door::new((2, 9), Orientation::North),
                Door::new((2, 0), Orientation::South),
            ],
        }
    }
}

pub struct RoomToInstantiate {
    pub room: Room,
    pub position: IVec2,
}

impl RoomToInstantiate {
    pub fn new<T: Into<IVec2>>(room: Room, position: T) -> Self {
        Self {
            room,
            position: position.into(),
        }
    }
}
