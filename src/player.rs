use bevy::math::vec3;
use bevy::time::Stopwatch;
use bevy::utils::Instant;
use bevy::{math::vec2, prelude::*};
use std::f32::consts::PI;

use crate::resources::*;
use crate::*;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct FireTimer(pub Stopwatch);
#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct SpawnInstant(Instant);
#[derive(Component)]
struct BulletDirection(Vec3);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init_player).add_systems(
            Update,
            (
                update_player_movement,
                update_player_rotation,
                update_player_fire,
                update_bullet,
            ),
        );
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
        FireTimer(Stopwatch::new()),
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

fn update_player_fire(
    mut commands: Commands,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    mut player_query: Query<(&Transform, &mut FireTimer), With<Player>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (player_transform, mut fire_timer) = player_query.single_mut();
    fire_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let bullet_direction = player_transform.local_y();
    let bullet_rotation = player_transform.rotation;

    if fire_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        fire_timer.0.reset();
        let dir = vec3(bullet_direction.x, bullet_direction.y, bullet_direction.z);
        let rotation_z = bullet_rotation.z;
        print!("rotation: {:?},", bullet_rotation.z);
        let cos_z = rotation_z.cos();
        print!("cos z: {:?},", cos_z);
        let sin_z = rotation_z.sin();
        println!("sin z: {:?},", sin_z);

        commands.spawn((
            SpriteBundle {
                texture: game_textures.player_laser.clone(),
                transform: Transform {
                    translation: vec3(dir.x, dir.y, 1.0),
                    rotation: bullet_rotation,
                    ..default()
                },
                ..default()
            },
            Bullet,
            BulletDirection(dir),
            SpawnInstant(Instant::now()),
        ));
        println!("x: {:?} y: {:?}", dir.x, dir.y);
    }
}

fn update_bullet(mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize() * Vec3::splat(BULLET_SPEED);
        t.translation.z = 10.0;
    }
}
