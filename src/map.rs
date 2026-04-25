use bevy::prelude::*;

pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cols = (5000.0 / 48.0) as i32;
    let rows = (5000.0 / 48.0) as i32;
    let tile_handle: Handle<Image> = asset_server.load("tile.png");

    for row in 0..rows {
        for col in 0..cols {
            let x = -2500.0 + col as f32 * 48.0;
            let y = -2500.0 + row as f32 * 48.0;

            commands.spawn((
                Sprite::from(tile_handle.clone()),
                Transform::from_xyz(x, y, -1.0),
            ));
        }
    }
}
