use crate::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct ResourcesPlugin;

#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);
#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    // explosion_layout: Handle<TextureAtlasLayout>,
    // explosion_texture: Handle<Image>,
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition(None))
            .add_systems(Startup, init_game_texture)
            .add_systems(Update, (update_cursor_position, close_on_esc));
    }
}

fn init_game_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        // explosion_layout,
        // explosion_texture: texture_handle,
    };
    commands.insert_resource(game_textures);
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = if let Ok(window) = window_query.get_single() {
        window
    } else {
        return;
    };

    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
