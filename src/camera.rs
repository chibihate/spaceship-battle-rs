use bevy::prelude::*;

pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
