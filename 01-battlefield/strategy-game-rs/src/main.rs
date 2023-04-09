use bevy::prelude::*;

fn create_battlefield_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tiles_handle = asset_server.load("Tiles/FullTileset.png");
    let tiles_atlas =
        TextureAtlas::from_grid(tiles_handle, Vec2::new(16.0, 16.0), 20, 20, None, None);
    let tiles_atlas_handle = texture_atlases.add(tiles_atlas);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteSheetBundle {
        texture_atlas: tiles_atlas_handle,
        sprite: TextureAtlasSprite::new(3),
        transform: Transform::from_scale(Vec3::splat(4.0)),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_battlefield_system)
        .run();
}
