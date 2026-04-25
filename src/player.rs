use bevy::prelude::*;
use std::f32::consts::PI;

use crate::components::Collider;
use crate::projectile::Projectile;

const PLAYER_RADIUS: f32 = 10.0;
const PLAYER_SPEED: f32 = 200.0;
const PROJECTILE_1_RADIUS: f32 = 2.0;
const PROJECTILE_2_RADIUS: f32 = 5.0;
const PROJECTILE_3_RADIUS: f32 = 10.0;

#[derive(Component)]
pub struct Player {
    weapon_type: WeaponType,
    weapon_facing: Vec2,
}

// TODO: Review weapon logic, range, area of attack, decay, etc
enum WeaponType {
    Melee, // TODO Improve this, has fields that don't really apply
    RadialBurst,
    HeavyRadial,
}

pub fn setup(
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

pub fn fire_weapon(
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
                let num_projectiles = 9.0;
                let angle_step = (2.0 * PI) / num_projectiles;
                for i in 0..9 {
                    let angle = angle_step * i as f32;
                    spawn_radial_burst_projectile(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        location,
                        angle,
                    );
                }
            }
            WeaponType::HeavyRadial => {
                let num_projectiles = 4.0;
                let angle_step = (2.0 * PI) / num_projectiles;
                for i in 0..4 {
                    let angle = angle_step * i as f32;
                    spawn_heavy_radial_projectile(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        location,
                        angle,
                    );
                }
            }
        }
    }
}

fn spawn_melee(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player: &Player,
    location: Vec3,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(15.0, 5.0))),
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
            lifetime: 0.1,
        },
    ));
}

fn spawn_radial_burst_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
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
            lifetime: 10.0,
        },
    ));
}

fn spawn_heavy_radial_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    location: Vec3,
    angle: f32,
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
            direction: Vec2::from_angle(angle),
            max_distance: 75.0,
            distance_traveled: 0.0,
            lifetime: 10.0,
        },
    ));
}

pub fn move_player(
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

pub fn update_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
