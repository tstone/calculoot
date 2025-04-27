use std::collections::HashSet;

pub use bevy::prelude::*;
use bevy::window::WindowResized;
use rand::prelude::IteratorRandom;
use rand::{Rng, SeedableRng};

use crate::seed::RngSeed;

pub struct BackgroundTiles;

impl Plugin for BackgroundTiles {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, on_window_resize);
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
    seed: Res<RngSeed>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let image: Handle<Image> = asset_server.load("grass-tileset.png");
    let atlas = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        8,
        5,
        None,
        None,
    ));
    commands.insert_resource(BackgroundTileset(image.id(), atlas.id()));

    match windows.single() {
        Ok(win) => render(
            win.width() as usize,
            win.height() as usize,
            image,
            atlas,
            seed.0,
            &mut commands,
        ),
        _ => {}
    }
}

fn on_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    background_tileset: Res<BackgroundTileset>,
    asset_server: Res<AssetServer>,
    seed: Res<RngSeed>,
    mut commands: Commands,
) {
    for e in resize_reader.read() {
        let image = asset_server.get_id_handle(background_tileset.0);
        let layout = asset_server.get_id_handle(background_tileset.1);

        match (image, layout) {
            (Some(image), Some(layout)) => render(
                e.width as usize,
                e.height as usize,
                image,
                layout,
                seed.0,
                &mut commands,
            ),
            _ => {}
        }
    }
}

fn render(
    width: usize,
    height: usize,
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    seed: u64,
    commands: &mut Commands,
) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let grass_start: usize = 0;
    let grass_texture_end = 8 * 2;
    let grass_end: usize = 8 * 4;
    let rocks_end = (8 * 5) - 3;

    let tile_width = 32;
    let tile_height = 32;
    let render_width: usize = (width as f32 / 3.0 / 32.0) as usize + 1;
    let render_height: usize = (height as f32 / 3.0 / 32.0) as usize + 1;

    let mut rocks: HashSet<usize> = HashSet::new();
    for i in grass_end..rocks_end {
        rocks.insert(i);
    }

    for x in 0..=render_width {
        for y in 0..=render_height {
            let index = if rng.random_bool(0.75) {
                0
            } else if rng.random_bool(0.8) {
                (grass_start..grass_texture_end).choose(&mut rng).unwrap()
            } else {
                (grass_texture_end..grass_end).choose(&mut rng).unwrap()
            };

            let x_pos = (x * tile_width) as f32;
            let y_pos = (y * tile_height) as f32;

            // grass
            commands.spawn((
                Sprite {
                    image: image.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: layout.clone(),
                        index,
                    }),
                    ..Default::default()
                },
                Transform::from_xyz(x_pos, y_pos, 0.0),
            ));

            let rem_rocks = rocks.clone();
            if x > 5 && rem_rocks.len() > 0 && rng.random_bool(0.01) {
                let index = rem_rocks.iter().choose(&mut rng).unwrap();
                rocks.remove(index);

                // rocks
                commands.spawn((
                    Sprite {
                        image: image.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: layout.clone(),
                            index: *index,
                        }),
                        ..Default::default()
                    },
                    Transform::from_xyz(x_pos, y_pos, 1.0),
                ));
            }
        }
    }
}

#[derive(Resource)]
pub struct BackgroundTileset(pub AssetId<Image>, pub AssetId<TextureAtlasLayout>);
