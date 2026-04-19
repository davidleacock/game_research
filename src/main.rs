use bevy::prelude::*;

const PLAYER_RADIUS: f32 = 10.0;
const PLAYER_SPEED: f32 = 200.0;
const ENEMY_RADIUS: f32 = 5.0;
const ENEMY_SPEED: f32 = 25.0;

const PROJECTILE_1_RADIUS: f32 = 2.0;
const PROJECTILE_2_RADIUS: f32 = 5.0;
const PROJECTILE_3_RADIUS: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, move_enemy)
        .add_systems(Update, detect_collisions)
        .add_systems(Update, fire_weapon)
        .add_systems(Update, move_projectiles)
        .add_systems(Update, detect_projectile_collisions)
        .run();
}

#[derive(Component)]
struct Player {
    weapon_type: WeaponType,
    facing: Vec2,
}

enum WeaponType {
    ShortDistance,
    MediumDistance,
    LongDistance,
}

#[derive(Component)]
struct Projectile {
    speed: f32,
    direction: Vec2,
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Collider {
    radius: f32,
}

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
        Player {
            weapon_type: WeaponType::ShortDistance,
            facing: Vec2::new(1.0, 0.0),
        },
        Collider {
            radius: PLAYER_RADIUS,
        },
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(ENEMY_RADIUS))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(300.0, 300.0, 0.0),
        Enemy,
        Collider {
            radius: ENEMY_RADIUS,
        },
    ));
}

fn fire_weapon(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player: Query<(&Transform, &Player), With<Player>>,
) {
    let Ok((player_transform, player)) = player.single() else {
        return;
    };

    let location = player_transform.translation;

    if keys.just_pressed(KeyCode::Space) {
        match player.weapon_type {
            WeaponType::ShortDistance => {
                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(PROJECTILE_1_RADIUS))),
                    MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
                    Transform::from_xyz(location.x, location.y, 0.0),
                    Collider { radius: PROJECTILE_1_RADIUS },
                    Projectile {
                        speed: 300.0,
                        direction: player.facing,
                    },
                ));
            }
            WeaponType::MediumDistance => {
                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(PROJECTILE_2_RADIUS))),
                    MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 1.0, 0.0))),
                    Transform::from_xyz(location.x, location.y, 0.0),
                    Collider { radius: PROJECTILE_2_RADIUS },
                    Projectile {
                        speed: 150.0,
                        direction: player.facing,
                    },
                ));
            }
            WeaponType::LongDistance => {
                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(PROJECTILE_3_RADIUS))),
                    MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 0.0, 1.0))),
                    Transform::from_xyz(location.x, location.y, 0.0),
                    Collider { radius: PROJECTILE_3_RADIUS },
                    Projectile {
                        speed: 75.0,
                        direction: player.facing,
                    },
                ));
            }
        }
    }
}

fn detect_collisions(
    mut enemy: Query<(&mut Transform, &Collider), With<Enemy>>,
    player: Query<(&Transform, &Collider), (With<Player>, Without<Enemy>)>,
) {
    let Ok((mut enemy_transform, enemy_collider)) = enemy.single_mut() else {
        return;
    };

    let Ok((player_transform, player_collider)) = player.single() else {
        return;
    };

    let distance = enemy_transform
        .translation
        .distance(player_transform.translation);

    if distance < player_collider.radius + enemy_collider.radius {
        let push_direction =
            (enemy_transform.translation - player_transform.translation).normalize_or_zero();
        enemy_transform.translation = player_transform.translation
            + push_direction * (player_collider.radius + enemy_collider.radius);
    }
}

fn detect_projectile_collisions(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &Collider), With<Projectile>>,
    enemies: Query<(&Transform, &Collider), With<Enemy>>
) {

    for (proj_entity, proj_transformer, proj_collider) in &projectiles {
        for (enemy_transform, enemy_collider) in &enemies {

            let distance = enemy_transform
                .translation
                .distance(proj_transformer.translation);

            if distance < proj_collider.radius + enemy_collider.radius {
                commands.entity(proj_entity).despawn();
                break;
            }
        }
    }
}

fn move_projectiles(
    time: Res<Time>,
    mut projectiles: Query<(&mut Transform, &Projectile)>,
) {
    let dt = time.delta_secs();
    for (mut transform, projectile) in &mut projectiles {
        transform.translation.y += projectile.direction.y * projectile.speed * dt;
        transform.translation.x += projectile.direction.x * projectile.speed * dt;
    }
}

fn move_enemy(
    time: Res<Time>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let Ok(mut enemy_transform) = enemy_query.single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let direction =
        (player_transform.translation - enemy_transform.translation).normalize_or_zero();

    enemy_transform.translation.x += ENEMY_SPEED * direction.x * time.delta_secs();
    enemy_transform.translation.y += ENEMY_SPEED * direction.y * time.delta_secs();
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let Ok((mut transform, mut player)) = query.single_mut() else {
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
        player.facing = direction;
    }

    if keys.pressed(KeyCode::Digit1) {
        player.weapon_type = WeaponType::ShortDistance;
    }

    if keys.pressed(KeyCode::Digit2) {
        player.weapon_type = WeaponType::MediumDistance;
    }

    if keys.pressed(KeyCode::Digit3) {
        player.weapon_type = WeaponType::LongDistance;
    }

    transform.translation.y += direction.y * PLAYER_SPEED * dt;
    transform.translation.x += direction.x * PLAYER_SPEED * dt;
}
