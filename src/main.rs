use background::BackgroundTiles;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use camera::CameraSetup;
use level::Levels;
use seed::SeedPlugin;
use sprite_animation::SpriteAnimationPlugin;

use std::collections::HashSet;

use eq_gen::{Equation, NumberType, OperationConfig, OperationType};
use rand::{prelude::*, random_range};
use rand_chacha::ChaCha8Rng;

mod background;
mod camera;
mod eq_gen;
mod level;
mod mode;
mod seed;
mod sprite_animation;

fn main() -> AppExit {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        name: Some("Game".to_string()),
                        resolution: (1920., 1080.).into(),
                        // mode: bevy::window::WindowMode::Fullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,calculoot=trace".to_string(),
                    level: Level::TRACE,
                    ..Default::default()
                }),
        )
        .add_plugins((
            SeedPlugin,
            SpriteAnimationPlugin,
            BackgroundTiles,
            CameraSetup,
            Levels,
        ))
        .run()
}
