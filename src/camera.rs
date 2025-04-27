use bevy::prelude::*;

pub struct CameraSetup;

impl Plugin for CameraSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn camera(mut commands: Commands) {
    commands.spawn((
        Msaa::Off,
        Camera2d,
        MainCamera,
        Projection::from(OrthographicProjection {
            scale: 1.0 / 3.0,
            viewport_origin: Vec2::new(0.0, 0.0),
            ..OrthographicProjection::default_2d()
        }),
    ));
}
