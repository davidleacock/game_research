use bevy::prelude::*;

const PLAYER_RADIUS: f32 = 10.0;
const PLAYER_SPEED: f32 = 200.0;
const ENEMY_RADIUS: f32 = 5.0;
const ENEMY_SPEED: f32 = 25.0;

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
struct Enemy;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(PLAYER_RADIUS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(ENEMY_RADIUS))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(300.0, 300.0, 0.0),
        Enemy,
    ));
}

fn move_enemy(
    time: Res<Time>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>
) {
    let Ok(mut enemy_transform) = enemy_query.single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let direction = (player_transform.translation - enemy_transform.translation).normalize_or_zero();

    enemy_transform.translation.x += ENEMY_SPEED * direction.x * time.delta_secs();
    enemy_transform.translation.y += ENEMY_SPEED * direction.y * time.delta_secs();
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    transform.translation.y += direction.y * PLAYER_SPEED * dt;
    transform.translation.x += direction.x * PLAYER_SPEED * dt;
}
