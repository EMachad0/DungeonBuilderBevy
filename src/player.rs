use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    let size = Vec2::new(16., 32.);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 0.5),
                custom_size: Some(size),
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_xyz(0., 0., 10.),
            ..Default::default()
        })
        .insert(Player);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 0.8),
                custom_size: Some(Vec2::new(32., 32.)),
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
            ..Default::default()
        });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
