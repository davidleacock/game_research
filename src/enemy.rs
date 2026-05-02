use bevy::prelude::*;
use rand::Rng;

use crate::components::Collider;
use crate::player::Player;

#[derive(Component)]
pub struct Enemy {
    pub current_health: f32,
    pub enemy_type: EnemyType,
}

enum EnemyType {
    Small,
    Medium,
    Large,
}

impl EnemyType {
    pub fn speed(&self) -> f32 {
        match self {
            EnemyType::Small => 50.0,
            EnemyType::Medium => 25.0,
            EnemyType::Large => 10.0
        }
    }

    pub fn size(&self) -> f32 {
        match self {
            EnemyType::Small => 8.0,
            EnemyType::Medium => 16.0,
            EnemyType::Large => 30.0
        }
    }

    pub fn max_health(&self) -> f32 {
        match self {
            EnemyType::Small => 50.0,
            EnemyType::Medium => 100.0,
            EnemyType::Large => 300.0
        }
    }
}

pub fn move_enemies(
    time: Res<Time>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &Enemy), With<Enemy>>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    for (mut enemy_transform, enemy) in &mut enemy_query {
        let direction =
            (player_transform.translation - enemy_transform.translation).normalize_or_zero();

        enemy_transform.translation.x += enemy.enemy_type.speed() * direction.x * time.delta_secs();
        enemy_transform.translation.y += enemy.enemy_type.speed() * direction.y * time.delta_secs();
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

pub fn check_input(
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

        let enemy_type = EnemyType::Medium;

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(enemy_type.size()))),
            MeshMaterial2d(materials.add(Color::linear_rgb(r, g, b))),
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                current_health: enemy_type.max_health(),
                enemy_type,
            },
            Collider { radius },
        ));
    }
}
