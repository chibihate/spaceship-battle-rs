use bevy::prelude::*;
use camera::CustomCameraPlugin;
use player::PlayerPlugin;
use resources::ResourcesPlugin;
use starship_battle::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Spaceship battle".into(),
                resizable: false,
                focused: true,
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CustomCameraPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
