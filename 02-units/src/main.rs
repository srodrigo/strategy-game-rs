use bevy::{prelude::*, window::WindowResolution};

const NUM_COLUMNS: usize = 20;
const NUM_ROWS: usize = 20;

enum TileType {
    Brown1,
    Brown2,
    Brown3,
    Brown4,
    Green1,
    Green2,
    Green3,
    Green4,
    BrownGreenUpper1,
    BrownGreenUpper2,
    BrownGreenUpper3,
    BrownGreenUpper5,
    BrownGreenUpper7,
    BrownGreenMiddle1,
    BrownGreenMiddle3,
    BrownGreenMiddle4,
    BrownGreenMiddle6,
    BrownGreenLower1,
    BrownGreenLower2,
    BrownGreenLower3,
    BrownGreenLower5,
    BrownGreenLower7,
}

struct Tile {
    index: usize,
}

impl Tile {
    fn from_type(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::Brown1 => Tile { index: 0 },
            TileType::Brown2 => Tile { index: 1 },
            TileType::Brown3 => Tile { index: 2 },
            TileType::Brown4 => Tile { index: 3 },
            TileType::Green1 => Tile {
                index: 2 * NUM_COLUMNS,
            },
            TileType::Green2 => Tile {
                index: 2 * NUM_COLUMNS + 1,
            },
            TileType::Green3 => Tile {
                index: 2 * NUM_COLUMNS + 2,
            },
            TileType::Green4 => Tile {
                index: 2 * NUM_COLUMNS + 3,
            },
            TileType::BrownGreenUpper1 => Tile {
                index: 7 * NUM_COLUMNS,
            },
            TileType::BrownGreenUpper2 => Tile {
                index: 7 * NUM_COLUMNS + 1,
            },
            TileType::BrownGreenUpper3 => Tile {
                index: 7 * NUM_COLUMNS + 2,
            },
            TileType::BrownGreenUpper5 => Tile {
                index: 7 * NUM_COLUMNS + 4,
            },
            TileType::BrownGreenUpper7 => Tile {
                index: 7 * NUM_COLUMNS + 6,
            },
            TileType::BrownGreenMiddle1 => Tile {
                index: 8 * NUM_COLUMNS,
            },
            TileType::BrownGreenMiddle3 => Tile {
                index: 8 * NUM_COLUMNS + 2,
            },
            TileType::BrownGreenMiddle4 => Tile {
                index: 8 * NUM_COLUMNS + 3,
            },
            TileType::BrownGreenMiddle6 => Tile {
                index: 8 * NUM_COLUMNS + 5,
            },
            TileType::BrownGreenLower1 => Tile {
                index: 9 * NUM_COLUMNS,
            },
            TileType::BrownGreenLower2 => Tile {
                index: 9 * NUM_COLUMNS + 1,
            },
            TileType::BrownGreenLower3 => Tile {
                index: 9 * NUM_COLUMNS + 2,
            },
            TileType::BrownGreenLower5 => Tile {
                index: 9 * NUM_COLUMNS + 4,
            },
            TileType::BrownGreenLower7 => Tile {
                index: 9 * NUM_COLUMNS + 6,
            },
        }
    }
}

fn create_battlefield_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    const TILE_SIZE: f32 = 16.0;

    let tiles_handle = asset_server.load("Tiles/FullTileset.png");
    let tiles_atlas = TextureAtlas::from_grid(
        tiles_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        NUM_COLUMNS,
        NUM_ROWS,
        None,
        None,
    );
    let tiles_atlas_handle = texture_atlases.add(tiles_atlas);

    const BATTLEFIELD_WIDTH_IN_TILES: usize = 13;
    const BATTLEFIELD_HEIGHT_IN_TILES: usize = 6;
    let tile_map: [[Tile; BATTLEFIELD_WIDTH_IN_TILES]; BATTLEFIELD_HEIGHT_IN_TILES] = [
        [
            Tile::from_type(TileType::Brown1),
            Tile::from_type(TileType::BrownGreenLower1),
            Tile::from_type(TileType::BrownGreenLower2),
            Tile::from_type(TileType::BrownGreenLower2),
            Tile::from_type(TileType::BrownGreenLower3),
            Tile::from_type(TileType::BrownGreenLower5),
            Tile::from_type(TileType::Brown2),
            Tile::from_type(TileType::BrownGreenLower1),
            Tile::from_type(TileType::BrownGreenLower3),
            Tile::from_type(TileType::BrownGreenLower5),
            Tile::from_type(TileType::BrownGreenLower1),
            Tile::from_type(TileType::BrownGreenLower3),
            Tile::from_type(TileType::Brown4),
        ],
        [
            Tile::from_type(TileType::Brown3),
            Tile::from_type(TileType::BrownGreenMiddle1),
            Tile::from_type(TileType::Green2),
            Tile::from_type(TileType::BrownGreenUpper2),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::Green3),
            Tile::from_type(TileType::BrownGreenLower2),
            Tile::from_type(TileType::Green3),
            Tile::from_type(TileType::Green2),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::BrownGreenMiddle3),
            Tile::from_type(TileType::BrownGreenMiddle1),
            Tile::from_type(TileType::BrownGreenLower3),
        ],
        [
            Tile::from_type(TileType::BrownGreenMiddle4),
            Tile::from_type(TileType::Green3),
            Tile::from_type(TileType::BrownGreenMiddle3),
            Tile::from_type(TileType::BrownGreenLower5),
            Tile::from_type(TileType::BrownGreenMiddle1),
            Tile::from_type(TileType::Green3),
            Tile::from_type(TileType::Green4),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::Green2),
            Tile::from_type(TileType::Green3),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::Green2),
            Tile::from_type(TileType::BrownGreenUpper3),
        ],
        [
            Tile::from_type(TileType::BrownGreenMiddle4),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::Green3),
            Tile::from_type(TileType::Green4),
            Tile::from_type(TileType::Green2),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::BrownGreenMiddle3),
            Tile::from_type(TileType::BrownGreenUpper7),
            Tile::from_type(TileType::BrownGreenUpper1),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::Green3),
            Tile::from_type(TileType::BrownGreenMiddle6),
        ],
        [
            Tile::from_type(TileType::Brown2),
            Tile::from_type(TileType::BrownGreenUpper1),
            Tile::from_type(TileType::Green4),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::Green3),
            Tile::from_type(TileType::Green2),
            Tile::from_type(TileType::BrownGreenUpper2),
            Tile::from_type(TileType::Green1),
            Tile::from_type(TileType::Green3),
            Tile::from_type(TileType::BrownGreenLower2),
            Tile::from_type(TileType::Green4),
            Tile::from_type(TileType::Green2),
            Tile::from_type(TileType::BrownGreenMiddle6),
        ],
        [
            Tile::from_type(TileType::Brown1),
            Tile::from_type(TileType::Brown4),
            Tile::from_type(TileType::BrownGreenUpper5),
            Tile::from_type(TileType::BrownGreenUpper1),
            Tile::from_type(TileType::BrownGreenUpper3),
            Tile::from_type(TileType::BrownGreenUpper1),
            Tile::from_type(TileType::BrownGreenLower7),
            Tile::from_type(TileType::BrownGreenUpper3),
            Tile::from_type(TileType::BrownGreenUpper5),
            Tile::from_type(TileType::BrownGreenUpper1),
            Tile::from_type(TileType::BrownGreenUpper2),
            Tile::from_type(TileType::BrownGreenUpper3),
            Tile::from_type(TileType::Brown2),
        ],
    ];

    const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;
    const HALF_BATTLEFIELD_WIDTH_IN_PIXELS: f32 =
        BATTLEFIELD_WIDTH_IN_TILES as f32 * TILE_SIZE / 2.0;
    const HALF_BATTLEFIELD_HEIGHT_IN_PIXELS: f32 =
        BATTLEFIELD_HEIGHT_IN_TILES as f32 * TILE_SIZE / 2.0;
    const WIDTH_CENTER_OFFSET: f32 = HALF_BATTLEFIELD_WIDTH_IN_PIXELS - HALF_TILE_SIZE;
    const HEIGHT_CENTER_OFFSET: f32 = HALF_BATTLEFIELD_HEIGHT_IN_PIXELS - HALF_TILE_SIZE;

    for (y, row) in tile_map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: tiles_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(col.index),
                transform: Transform {
                    translation: Vec3 {
                        x: x as f32 * TILE_SIZE - WIDTH_CENTER_OFFSET,
                        y: y as f32 * TILE_SIZE - HEIGHT_CENTER_OFFSET,
                        z: 0.0,
                    },
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Strategy Game in Rust".to_string(),
                        resolution: WindowResolution::new(960.0, 540.0)
                            .with_scale_factor_override(4.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_startup_system(create_battlefield_system)
        .run();
}
