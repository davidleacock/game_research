use bevy::prelude::*;
use rand::Rng;
use std::char::from_u32;
use std::f32::consts::PI;
use std::ops::Div;
use log::__private_api::loc;

const PLAYER_RADIUS: f32 = 10.0;
const PLAYER_SPEED: f32 = 200.0;
const ENEMY_SPEED: f32 = 25.0;

const PROJECTILE_1_RADIUS: f32 = 2.0;
const PROJECTILE_2_RADIUS: f32 = 5.0;
const PROJECTILE_3_RADIUS: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, move_enemies)
        .add_systems(Update, detect_collisions)
        .add_systems(Update, fire_weapon)
        .add_systems(Update, move_projectiles)
        .add_systems(Update, detect_projectile_collisions)
        .add_systems(Update, check_input)
        .run();
}

#[derive(Component)]
struct Player {
    weapon_type: WeaponType,
    weapon_facing: Vec2,
}

enum WeaponType {
    Melee,
    RadialBurst,
    HeavyRadial,
}

#[derive(Component)]
struct Projectile {
    speed: f32,
    direction: Vec2,
    max_distance: f32,
    distance_traveled: f32,
}

#[derive(Component)]
struct Enemy {
    health: f32,
}

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
            weapon_type: WeaponType::Melee,
            weapon_facing: Vec2::new(1.0, 0.0),
        },
        Collider {
            radius: PLAYER_RADIUS,
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
            WeaponType::Melee => {
                spawn_melee(&mut commands, &mut meshes, &mut materials, player, location);
            }
            WeaponType::RadialBurst => {
                let num_projectiles = 10.0;
                let angle_step = (2.0 * PI) / num_projectiles;
                for i in 1..10 {
                    let angle = angle_step * i as f32;
                    spawn_radial_burst_projectile(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        player,
                        location,
                        angle,
                    );
                }
            }
            WeaponType::HeavyRadial => {
                spawn_heavy_radial_projectile(&mut commands, meshes, materials, player, location);
            }
        }
    }
}

fn spawn_heavy_radial_projectile(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player: &Player,
    location: Vec3,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(PROJECTILE_3_RADIUS))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(location.x, location.y, 0.0),
        Collider {
            radius: PROJECTILE_3_RADIUS,
        },
        Projectile {
            speed: 75.0,
            direction: player.weapon_facing,
            max_distance: 75.0,
            distance_traveled: 0.0,
        },
    ));
}

fn spawn_radial_burst_projectile(
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    player: &Player,
    location: Vec3,
    angle: f32,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(PROJECTILE_2_RADIUS))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 1.0, 0.0))),
        Transform::from_xyz(location.x, location.y, 0.0),
        Collider {
            radius: PROJECTILE_2_RADIUS,
        },
        Projectile {
            speed: 150.0,
            direction: Vec2::from_angle(angle),
            max_distance: 350.0,
            distance_traveled: 0.0,
        },
    ));
}

fn spawn_melee(
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    player: &Player,
    location: Vec3,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(PROJECTILE_1_RADIUS))),
        MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(location.x, location.y, 0.0),
        Collider {
            radius: PROJECTILE_1_RADIUS,
        },
        Projectile {
            speed: 300.0,
            direction: player.weapon_facing,
            max_distance: 25.0,
            distance_traveled: 0.0,
        },
    ));
}

fn detect_collisions(
    mut enemy: Query<(&mut Transform, &Collider), With<Enemy>>,
    player: Query<(&Transform, &Collider), (With<Player>, Without<Enemy>)>,
) {
    let Ok((player_transform, player_collider)) = player.single() else {
        return;
    };

    for (mut enemy_transform, enemy_collider) in &mut enemy {
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
}

fn detect_projectile_collisions(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &Collider), With<Projectile>>,
    mut enemies: Query<(&Transform, &Collider, &mut Enemy, Entity), With<Enemy>>,
) {
    for (proj_entity, proj_transformer, proj_collider) in &projectiles {
        for (enemy_transform, enemy_collider, mut enemy, enemy_entity) in &mut enemies {
            let distance = enemy_transform
                .translation
                .distance(proj_transformer.translation);

            if distance < proj_collider.radius + enemy_collider.radius {
                commands.entity(proj_entity).despawn();
                enemy.health -= 25.0;
                if enemy.health <= 0.0 {
                    commands.entity(enemy_entity).despawn();
                }
                break;
            }
        }
    }
}

fn move_projectiles(
    time: Res<Time>,
    mut projectiles: Query<(&mut Transform, &mut Projectile, Entity)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (mut transform, mut projectile, entity) in &mut projectiles {
        if projectile.distance_traveled >= projectile.max_distance {
            commands.entity(entity).despawn();
            continue;
        }

        transform.translation.y += projectile.direction.y * projectile.speed * dt;
        transform.translation.x += projectile.direction.x * projectile.speed * dt;

        projectile.distance_traveled += projectile.speed * dt;
    }
}

fn move_enemies(
    time: Res<Time>,
    mut enemy: Query<&mut Transform, With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    for mut enemy_transform in &mut enemy {
        let direction =
            (player_transform.translation - enemy_transform.translation).normalize_or_zero();

        enemy_transform.translation.x += ENEMY_SPEED * direction.x * time.delta_secs();
        enemy_transform.translation.y += ENEMY_SPEED * direction.y * time.delta_secs();
    }
}

fn check_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if keys.pressed(KeyCode::KeyE) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-640.0..640.0);
        let y = rng.gen_range(-360.0..360.0);
        let r = rng.gen_range(0.0..1.0);
        let g = rng.gen_range(0.0..1.0);
        let b = rng.gen_range(0.0..1.0);
        let radius = rng.gen_range(2.5..25.0);

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(Color::linear_rgb(r, g, b))),
            Transform::from_xyz(x, y, 0.0),
            Enemy { health: 100.0 },
            Collider { radius },
        ));
    }
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let Ok((mut transform, mut player)) = player.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let mut direction = Vec2::ZERO;
    let mut weapon_facing = Vec2::ZERO;

    if keys.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
        weapon_facing.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
        weapon_facing.x += 1.0;
    }
    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    if weapon_facing != Vec2::ZERO {
        weapon_facing = weapon_facing.normalize();
        player.weapon_facing = weapon_facing;
    }

    if keys.pressed(KeyCode::Digit1) {
        player.weapon_type = WeaponType::Melee;
    }

    if keys.pressed(KeyCode::Digit2) {
        player.weapon_type = WeaponType::RadialBurst;
    }

    if keys.pressed(KeyCode::Digit3) {
        player.weapon_type = WeaponType::HeavyRadial;
    }

    transform.translation.y += direction.y * PLAYER_SPEED * dt;
    transform.translation.x += direction.x * PLAYER_SPEED * dt;
}
