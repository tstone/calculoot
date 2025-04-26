use bevy::prelude::*;

pub struct CameraSetup;

impl Plugin for CameraSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera);
    }
}

fn camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}
