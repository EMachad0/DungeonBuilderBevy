use bevy::math::IVec2;
use rand::prelude::*;
use std::collections::VecDeque;

use crate::dungeon::door::Door;
use crate::dungeon::dungeon::Dungeon;
use crate::dungeon::dungeon_level::DungeonLevel;
use crate::dungeon::room::{RoomToInstantiate, RoomType};
use crate::orientation::Orientation;

pub struct DungeonBuilder {
    pub max_tries: usize,
    pub number_of_rooms: usize,
    pub level: DungeonLevel,
}

pub struct AvailableDoor {
    pub door: Door,
    pub global_position: IVec2,
}

impl DungeonBuilder {
    pub fn build(&self) -> Dungeon {
        let rng = &mut thread_rng();

        let DungeonLevel {
            rooms: available_rooms,
        } = &self.level;

        let entrance = available_rooms
            .iter()
            .filter(|room| room.room_type == RoomType::Entrance)
            .choose(rng)
            .expect("Entrance room not found");

        let mut available_doors = {
            let available_doors = entrance
                .doors
                .iter()
                .map(|door| AvailableDoor {
                    door: door.to_owned(),
                    global_position: door.position,
                })
                .collect::<Vec<_>>();
            VecDeque::from(available_doors)
        };
        let mut rooms_to_instantiate = vec![RoomToInstantiate::new(entrance.to_owned(), (0, 0))];

        while rooms_to_instantiate.len() < self.number_of_rooms {
            let AvailableDoor {
                door: parent_door,
                global_position: parent_door_position,
            } = available_doors
                .pop_front()
                .expect("No more doors available");

            let child_room = available_rooms
                .iter()
                .filter(|room| {
                    room.doors
                        .iter()
                        .any(|d| d.orientation == parent_door.orientation.opposite())
                })
                .choose(rng)
                .expect("No room found for door");

            let offset = match parent_door.orientation {
                Orientation::North => IVec2::new(0, 1),
                Orientation::East => IVec2::new(1, 0),
                Orientation::South => IVec2::new(0, -1),
                Orientation::West => IVec2::new(-1, 0),
            };

            let child_door = child_room
                .doors
                .iter()
                .find(|d| d.orientation == parent_door.orientation.opposite())
                .unwrap();

            let child_room_position = parent_door_position + offset - child_door.position;

            let room_to_instantiate =
                RoomToInstantiate::new(child_room.to_owned(), child_room_position);

            if check_collision(&room_to_instantiate, &rooms_to_instantiate) {
                continue;
            }

            rooms_to_instantiate.push(room_to_instantiate);

            let child_room_available_doors = child_room
                .doors
                .iter()
                .filter(|d| *d != child_door)
                .map(|door| AvailableDoor {
                    door: door.to_owned(),
                    global_position: child_room_position + door.position,
                })
                .collect::<Vec<_>>();
            available_doors.extend(child_room_available_doors);
        }

        Dungeon {
            rooms: rooms_to_instantiate,
        }
    }
}

fn check_collision(room: &RoomToInstantiate, rooms: &[RoomToInstantiate]) -> bool {
    let room_position = room.position;
    let room_size = room.room.size;

    let room_left = room_position.x;
    let room_right = room_position.x + room_size.x;
    let room_bottom = room_position.y;
    let room_top = room_position.y + room_size.y;

    rooms.iter().any(|other| {
        let other_position = other.position;
        let other_size = other.room.size;

        let other_left = other_position.x;
        let other_right = other_position.x + other_size.x;
        let other_bottom = other_position.y;
        let other_top = other_position.y + other_size.y;

        room_left < other_right
            && room_right > other_left
            && room_bottom < other_top
            && room_top > other_bottom
    })
}
