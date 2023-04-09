use bevy::prelude::*;

enum TileType {
    Brown1,
    Brown2,
    Brown3,
    Brown4,
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
        }
    }
}

fn create_battlefield_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    const NUM_COLUMNS: usize = 20;
    const NUM_ROWS: usize = 20;
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

    const SCALE: f32 = 4.0;
    let tile_map: [[Tile; 2]; 2] = [
        [
            Tile::from_type(TileType::Brown1),
            Tile::from_type(TileType::Brown3),
        ],
        [
            Tile::from_type(TileType::Brown2),
            Tile::from_type(TileType::Brown4),
        ],
    ];

    for (y, row) in tile_map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: tiles_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(col.index),
                transform: Transform {
                    translation: Vec3 {
                        x: x as f32 * TILE_SIZE * SCALE,
                        y: y as f32 * TILE_SIZE * SCALE,
                        z: 0.0,
                    },
                    scale: Vec3::splat(SCALE),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_battlefield_system)
        .run();
}
