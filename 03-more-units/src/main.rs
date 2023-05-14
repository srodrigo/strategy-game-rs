use bevy::{prelude::*, window::WindowResolution};

type TilemapDimensions = [[Tile; 13]; 6];

struct Tilemap {
    data: TilemapDimensions,
    num_columns: usize,
    num_rows: usize,
}

impl Tilemap {
    pub fn new(data: TilemapDimensions) -> Self {
        let num_columns = data.get(0).unwrap().len();
        let num_rows = data.len();

        Self {
            data,
            num_columns,
            num_rows,
        }
    }
}

#[derive(Resource)]
struct Battlefield {
    tile_size: f32,
    tilemap: Tilemap,
}

impl Battlefield {
    pub fn default() -> Self {
        Self {
            tile_size: 16.0,
            tilemap: Tilemap::new([
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
            ]),
        }
    }

    pub fn to_battlefield_coordinates(&self, x: f32, y: f32, z: f32) -> Vec3 {
        let half_tile_size: f32 = self.tile_size / 2.0;
        let half_battlefield_width_in_pixels: f32 =
            self.tilemap.num_columns as f32 * self.tile_size / 2.0;
        let half_battlefield_height_in_pixels: f32 =
            self.tilemap.num_rows as f32 * self.tile_size / 2.0;
        let width_center_offset: f32 = half_battlefield_width_in_pixels - half_tile_size;
        let height_center_offset: f32 = half_battlefield_height_in_pixels - half_tile_size;

        Vec3::new(x - width_center_offset, y - height_center_offset, z)
    }
}

const BATTLEFIELD_NUM_COLUMNS: usize = 20;
const BATTLEFIELD_NUM_ROWS: usize = 20;

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
                index: 2 * BATTLEFIELD_NUM_COLUMNS,
            },
            TileType::Green2 => Tile {
                index: 2 * BATTLEFIELD_NUM_COLUMNS + 1,
            },
            TileType::Green3 => Tile {
                index: 2 * BATTLEFIELD_NUM_COLUMNS + 2,
            },
            TileType::Green4 => Tile {
                index: 2 * BATTLEFIELD_NUM_COLUMNS + 3,
            },
            TileType::BrownGreenUpper1 => Tile {
                index: 7 * BATTLEFIELD_NUM_COLUMNS,
            },
            TileType::BrownGreenUpper2 => Tile {
                index: 7 * BATTLEFIELD_NUM_COLUMNS + 1,
            },
            TileType::BrownGreenUpper3 => Tile {
                index: 7 * BATTLEFIELD_NUM_COLUMNS + 2,
            },
            TileType::BrownGreenUpper5 => Tile {
                index: 7 * BATTLEFIELD_NUM_COLUMNS + 4,
            },
            TileType::BrownGreenUpper7 => Tile {
                index: 7 * BATTLEFIELD_NUM_COLUMNS + 6,
            },
            TileType::BrownGreenMiddle1 => Tile {
                index: 8 * BATTLEFIELD_NUM_COLUMNS,
            },
            TileType::BrownGreenMiddle3 => Tile {
                index: 8 * BATTLEFIELD_NUM_COLUMNS + 2,
            },
            TileType::BrownGreenMiddle4 => Tile {
                index: 8 * BATTLEFIELD_NUM_COLUMNS + 3,
            },
            TileType::BrownGreenMiddle6 => Tile {
                index: 8 * BATTLEFIELD_NUM_COLUMNS + 5,
            },
            TileType::BrownGreenLower1 => Tile {
                index: 9 * BATTLEFIELD_NUM_COLUMNS,
            },
            TileType::BrownGreenLower2 => Tile {
                index: 9 * BATTLEFIELD_NUM_COLUMNS + 1,
            },
            TileType::BrownGreenLower3 => Tile {
                index: 9 * BATTLEFIELD_NUM_COLUMNS + 2,
            },
            TileType::BrownGreenLower5 => Tile {
                index: 9 * BATTLEFIELD_NUM_COLUMNS + 4,
            },
            TileType::BrownGreenLower7 => Tile {
                index: 9 * BATTLEFIELD_NUM_COLUMNS + 6,
            },
        }
    }
}

fn create_battlefield_system(
    battlefield: Res<Battlefield>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let tiles_handle = asset_server.load("Tiles/FullTileset.png");
    let tiles_atlas = TextureAtlas::from_grid(
        tiles_handle,
        Vec2::new(battlefield.tile_size, battlefield.tile_size),
        BATTLEFIELD_NUM_COLUMNS,
        BATTLEFIELD_NUM_ROWS,
        None,
        None,
    );
    let tiles_atlas_handle = texture_atlases.add(tiles_atlas);

    for (y, row) in battlefield.tilemap.data.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: tiles_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(col.index),
                transform: Transform {
                    translation: battlefield.to_battlefield_coordinates(
                        x as f32 * battlefield.tile_size,
                        y as f32 * battlefield.tile_size,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn create_units_system(
    battlefield: Res<Battlefield>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    const SPRITE_SIZE: f32 = 32.0;
    const NUM_COLUMNS: usize = 4;
    const NUM_ROWS: usize = 4;

    let archer_blue_light_handle = asset_server.load("Sprite Sheets/Archer/Archer_Blue1.png");
    let archer_atlas = TextureAtlas::from_grid(
        archer_blue_light_handle,
        Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
        NUM_COLUMNS,
        NUM_ROWS,
        None,
        None,
    );
    let archer_atlas_handle = texture_atlases.add(archer_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: archer_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform {
            translation: battlefield.to_battlefield_coordinates(
                0.0 * SPRITE_SIZE + SPRITE_SIZE / 4.0,
                0.0 * SPRITE_SIZE + SPRITE_SIZE / 4.0,
                1.0,
            ),
            ..default()
        },
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(Battlefield::default())
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
        .add_startup_system(create_units_system)
        .run();
}
