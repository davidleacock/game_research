use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, move_enemy)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy {
    direction: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 10.0;

    commands.spawn(Camera2d::default());
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Player,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Enemy { direction: 1.0 },
    ));
}

fn move_enemy(mut query: Query<(&mut Transform, &mut Enemy)>) {
    let Ok((mut transform, mut enemy)) = query.single_mut() else {
        return;
    };

    transform.translation.y += 2.0 * enemy.direction;

    if transform.translation.y >= 150.0 || transform.translation.y <= -150.0 {
        enemy.direction *= -1.0;
    }
}

fn move_player(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    if keys.pressed(KeyCode::ArrowDown) && keys.pressed(KeyCode::ArrowLeft) {
        transform.translation.y -= 10.0;
        transform.translation.x -= 10.0;
    } else if keys.pressed(KeyCode::ArrowDown) && keys.pressed(KeyCode::ArrowRight) {
        transform.translation.y -= 10.0;
        transform.translation.x += 10.0;
    } else if keys.pressed(KeyCode::ArrowUp) && keys.pressed(KeyCode::ArrowRight) {
        transform.translation.y += 10.0;
        transform.translation.x += 10.0;
    } else if keys.pressed(KeyCode::ArrowUp) && keys.pressed(KeyCode::ArrowLeft) {
        transform.translation.y += 10.0;
        transform.translation.x -= 10.0;
    } else if keys.pressed(KeyCode::ArrowUp) {
        transform.translation.y += 10.0;
    } else if keys.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= 10.0;
    } else if keys.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 10.0;
    } else if keys.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 10.0;
    }
}
