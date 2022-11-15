use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
    }
}

/// Marker for the main game camera entity
#[derive(Component)]
struct GameCamera;

/// Spawn the camera
fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(GameCamera);
}
