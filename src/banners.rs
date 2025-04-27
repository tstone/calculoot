use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;

pub struct BannersPlugin;

impl Plugin for BannersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_assets);
        app.add_observer(render_board);
    }
}

fn init_assets(asset_server: Res<AssetServer>) {
    let _image: Handle<Image> = asset_server.load("board.png");
}

#[derive(Component, Debug)]
pub struct Board {
    pub text: String,
}

impl Board {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

fn render_board(
    trigger: Trigger<OnAdd, Board>,
    boards: Query<&Board>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let board = boards.get(trigger.target()).unwrap();
    let image: Handle<Image> = asset_server.load("board.png");
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(31, 33),
        3,
        3,
        None,
        None,
    ));

    debug!(
        "board image loaded? {:?}",
        asset_server.dependency_load_state(&image)
    );

    let win_size = match windows.single() {
        Ok(win) => (win.width(), win.height()),
        _ => (0.0, 0.0), // what to do here?
    };

    let tile_width = 31.0;
    let tile_height = 33.0;
    let win_width = win_size.0 / 3.0;
    let win_height = win_size.0 / 3.0;

    let x_pos_start = win_width / 2.0 - (tile_width * 3.0);
    let y_pos_start = (win_height / 2.0) - (tile_height * 0.75);
    let banner_width = tile_width * 8.0;
    let banner_height = tile_width * 2.0;
    let grayish_blue = Color::srgb(51.0 / 255.0, 50.0 / 255.0, 61.0 / 255.0);

    commands
        .entity(trigger.target())
        .insert((
            Transform::from_xyz(x_pos_start, y_pos_start, 5.0),
            Visibility::Visible,
        ))
        .with_child((
            TextFont {
                font: asset_server.load("monogram-extended.ttf"),
                font_size: 32.0,
                font_smoothing: bevy::text::FontSmoothing::None,
                ..Default::default()
            },
            Text2d::new(board.text.clone()),
            TextColor(grayish_blue),
            Transform::from_xyz(banner_width * 0.45, tile_height * 0.55, 0.0),
        ))
        .with_child((
            Mesh2d(meshes.add(Rectangle::new(banner_width, banner_height))),
            MeshMaterial2d(materials.add(grayish_blue.with_alpha(0.33))),
            Transform::from_xyz(banner_width * 0.47, 6.0, -1.0),
        ));

    for x in 0..8 {
        let x_pos = x as f32 * tile_width;
        for y in 0..2 {
            let y_pos = y as f32 * tile_height;
            let index = if x == 0 && y == 0 {
                BoardTiles::BottomLeft
            } else if x == 0 && y == 1 {
                BoardTiles::TopLeft
            } else if x == 7 && y == 0 {
                BoardTiles::BottomRight
            } else if x == 7 && y == 1 {
                BoardTiles::TopRight
            } else if y == 0 {
                BoardTiles::Bottom
            } else if y == 1 {
                BoardTiles::Top
            } else {
                BoardTiles::Middle
            };

            commands.entity(trigger.target()).with_child((
                Sprite {
                    image: image.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: layout.clone(),
                        index: index as usize,
                    }),
                    ..Default::default()
                },
                Transform::from_xyz(x_pos, y_pos, 0.0),
            ));
        }
    }

    // TODO: rename "board" to "box banner"
}

#[derive(Debug)]
#[repr(u16)]
enum BoardTiles {
    TopLeft = 0,
    Top = 1,
    TopRight = 2,
    Left = 3,
    Middle = 4,
    Right = 5,
    BottomLeft = 6,
    Bottom = 7,
    BottomRight = 8,
}
