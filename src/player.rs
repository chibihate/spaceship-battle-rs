use bevy::{math::vec2, prelude::*};
use std::f32::consts::PI;

use crate::resources::*;
use crate::*;

#[derive(Component)]
struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init_player)
            .add_systems(Update, (update_player_movement, update_player_rotation));
    }
}

fn init_player(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands.spawn((
        SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        },
        Player,
    ));
}

fn update_player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut velocity = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        velocity.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        velocity.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= 1.0;
    }

    let mut player_transform = player_query.single_mut();

    player_transform.translation.x += velocity.x * SPEED_PLAYER;
    player_transform.translation.y += velocity.y * SPEED_PLAYER;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((vec2(BOUNDS.0, BOUNDS.1) / 2.0, 0.0));
    player_transform.translation = player_transform.translation.min(extents).max(-extents);
}

fn update_player_rotation(
    cursor_pos: Res<CursorPosition>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };

    if player_pos != cursor_pos {
        let mut player_transform = player_query.single_mut();
        let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI / 2.0;
        player_transform.rotation = Quat::from_rotation_z(angle);
    }
}
