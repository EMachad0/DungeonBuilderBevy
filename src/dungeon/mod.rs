use bevy::prelude::*;

use crate::dungeon::dungeon_builder::DungeonBuilder;
use crate::dungeon::room::{Room, RoomToInstantiate};
use crate::tiled::{TiledMap, TiledMapBundle};
use crate::GameState;

mod door;
mod dungeon;
mod dungeon_builder;
mod dungeon_level;
mod room;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_dungeon.in_schedule(OnEnter(GameState::Playing)));

        #[cfg(debug_assertions)]
        {
            app.add_system(create_dungeon.run_if(recreate_dungeon_key_just_pressed));
        }
    }
}

pub fn create_dungeon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_rooms: Query<Entity, With<Room>>,
) {
    let builder = DungeonBuilder {
        number_of_rooms: 3,
        level: dungeon_level::DungeonLevel::level_1(),
        max_tries: 10,
    };

    for entity in existing_rooms.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let dungeon = builder.build();
    for RoomToInstantiate { room, position } in dungeon.rooms {
        let room_asset: Handle<TiledMap> = asset_server.load(room.tilemap_path.as_ref());
        commands.spawn((
            TiledMapBundle {
                tiled_map: room_asset,
                transform: Transform::from_xyz(
                    position.x as f32 * 16.,
                    position.y as f32 * 16.,
                    0.,
                ),
                ..Default::default()
            },
            room,
        ));
    }
}

fn recreate_dungeon_key_just_pressed(input: Res<Input<KeyCode>>) -> bool {
    input.just_pressed(KeyCode::R)
}
