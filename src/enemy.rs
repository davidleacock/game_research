use bevy::prelude::*;
use rand::Rng;

use crate::components::Collider;
use crate::player::Player;

const ENEMY_SPEED: f32 = 25.0;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

pub fn move_enemies(
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

pub fn detect_collisions(
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

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(Color::linear_rgb(r, g, b))),
            Transform::from_xyz(x, y, 0.0),
            Enemy { health: 100.0 },
            Collider { radius },
        ));
    }
}
