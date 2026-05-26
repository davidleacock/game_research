use EnemyState::Roam;
use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::{components::Collider, enemy::EnemyState::Chase, player::Player};

#[derive(Component)]
pub struct Enemy {
    pub current_health: f32,
    pub enemy_type: EnemyType,
    state: EnemyState,
    roam_direction: Vec2,
    roam_distance_remaining: f32,
}

enum EnemyState {
    Roam,
    Chase,
}

#[derive(Copy, Clone)]
pub enum EnemyType {
    Small,
    Medium,
    Large,
}

impl EnemyType {
    pub fn speed(&self) -> f32 {
        match self {
            EnemyType::Small => 50.0,
            EnemyType::Medium => 25.0,
            EnemyType::Large => 10.0,
        }
    }

    pub fn size(&self) -> f32 {
        match self {
            EnemyType::Small => 8.0,
            EnemyType::Medium => 16.0,
            EnemyType::Large => 30.0,
        }
    }

    pub fn max_health(&self) -> f32 {
        match self {
            EnemyType::Small => 50.0,
            EnemyType::Medium => 100.0,
            EnemyType::Large => 300.0,
        }
    }
}

#[derive(Event)]
pub struct EnemyKilled {
    pub position: Vec3,
}

pub fn move_enemies(
    time: Res<Time>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let mut rng = thread_rng();

    for (mut enemy_transform, mut enemy) in &mut enemy_query {
        if enemy_transform
            .translation
            .distance(player_transform.translation)
            < 300.0
        {
            enemy.state = Chase
        } else {
            enemy.state = Roam
        }

        match enemy.state {
            Roam => {
                if enemy.roam_distance_remaining <= 0.0 {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let direction = Vec2::from_angle(angle as f32);
                    let distance = rng.gen_range(10..100);

                    enemy.roam_direction = direction;
                    enemy.roam_distance_remaining = distance as f32;
                }

                enemy_transform.translation.x +=
                    enemy.enemy_type.speed() * enemy.roam_direction.x * time.delta_secs();
                enemy_transform.translation.y +=
                    enemy.enemy_type.speed() * enemy.roam_direction.y * time.delta_secs();

                enemy.roam_distance_remaining -= enemy.enemy_type.speed() * time.delta_secs();
            }
            Chase => {
                let direction = (player_transform.translation - enemy_transform.translation)
                    .normalize_or_zero();

                enemy_transform.translation.x +=
                    enemy.enemy_type.speed() * direction.x * time.delta_secs();
                enemy_transform.translation.y +=
                    enemy.enemy_type.speed() * direction.y * time.delta_secs();
            }
        }
    }
}

pub fn detect_collisions(
    player: Query<(&Transform, &Collider), (With<Player>, Without<Enemy>)>,
    mut enemy: Query<(&mut Transform, &Collider), With<Enemy>>,
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

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let num_clusters = 10;
    let mut rng = rand::thread_rng();

    for _ in 0..num_clusters {
        let cluster_x = rng.gen_range(-2500.0..2500.0);
        let cluster_y = rng.gen_range(-2500.0..2500.0);
        let num_enemies = rng.gen_range(3..10);
        for _ in 0..num_enemies {
            let half_size = 500.0;
            let enemy_x = cluster_x + rng.gen_range(-half_size..half_size);
            let enemy_y = cluster_y + rng.gen_range(-half_size..half_size);

            let enemy_type = match rng.gen_range(0..3) {
                0 => EnemyType::Small,
                1 => EnemyType::Medium,
                _ => EnemyType::Large,
            };

            let r = rng.gen_range(0.0..1.0);
            let g = rng.gen_range(0.0..1.0);
            let b = rng.gen_range(0.0..1.0);

            let direction = rng.gen_range(0.0..360.0);
            let initial_roam_distance = rng.gen_range(100.0..500.0);

            commands.spawn((
                Mesh2d(meshes.add(Circle::new(enemy_type.size()))),
                MeshMaterial2d(materials.add(Color::linear_rgb(r, g, b))),
                Transform::from_xyz(enemy_x, enemy_y, 0.0),
                Enemy {
                    current_health: enemy_type.max_health(),
                    enemy_type,
                    state: EnemyState::Roam,
                    roam_direction: Vec2::from_angle(direction),
                    roam_distance_remaining: initial_roam_distance,
                },
                Collider {
                    radius: enemy_type.size(),
                },
            ));
        }
    }
}

pub fn debug_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}
