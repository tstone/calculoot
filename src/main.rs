use background::BackgroundTiles;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use camera::CameraSetup;
use seed::SeedPlugin;
use sprite_animation::SpriteAnimationPlugin;

use std::collections::HashSet;

use eq_gen::{Equation, NumberType, OperationConfig, OperationType};
use rand::{prelude::*, random_range};
use rand_chacha::ChaCha8Rng;

mod background;
mod camera;
mod eq_gen;
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
        ))
        .run()
}

fn problems() {
    let seed = random_range(0..u64::MAX);
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let cfg = OperationConfig {
        answer_min: 1,
        answer_max: 20,
        value_min: -20,
        value_max: 40,
        allowed_numerics: HashSet::from([NumberType::Whole, NumberType::Negative]),
        allowed_operations: HashSet::from([
            OperationType::Add,
            OperationType::Subtract,
            OperationType::Multiply,
            OperationType::Divide,
        ]),
    };

    for _ in 0..10 {
        // let op_count = (1..=5).choose(&mut rng).unwrap();
        if let Some(eq) = Equation::rnd_compound(&cfg, 5, &mut rng) {
            println!("{} (measured difficulty: {})", eq, eq.difficulty());
        }
    }
}
