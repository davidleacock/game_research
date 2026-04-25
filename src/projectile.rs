use bevy::prelude::*;

use crate::components::Collider;
use crate::enemy::Enemy;

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub direction: Vec2,
    pub max_distance: f32,
    pub distance_traveled: f32,
    pub lifetime: f32,
}

pub fn move_projectiles(
    time: Res<Time>,
    mut projectiles: Query<(&mut Transform, &mut Projectile, Entity)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (mut transform, mut projectile, entity) in &mut projectiles {
        if projectile.distance_traveled >= projectile.max_distance || projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        projectile.lifetime -= dt;

        transform.translation.y += projectile.direction.y * projectile.speed * dt;
        transform.translation.x += projectile.direction.x * projectile.speed * dt;

        projectile.distance_traveled += projectile.speed * dt;
    }
}

pub fn detect_projectile_collisions(
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
