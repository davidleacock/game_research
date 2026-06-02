use bevy::prelude::*;

use crate::player::PlayerStats;
use crate::{components::Collider, enemy::EnemyKilled, player::Player};

const PICKUP_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Pickup {
    state: PickupState,
    pickup_type: PickupType,
}

pub enum PickupState {
    Dropped,
    Bounce { distance_remaining: f32 },
    Homing,
}

pub enum PickupType {
    Gem, // come up with better name
}

pub fn on_enemy_killed(
    trigger: On<EnemyKilled>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let position = trigger.event().position;

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(position.x, position.y, 0.0),
        Pickup {
            state: PickupState::Dropped,
            pickup_type: PickupType::Gem,
        },
        Collider { radius: 10.0 },
    ));
}

pub fn detect_collisions(
    mut commands: Commands,
    mut player: Query<(&Transform, &Collider, &mut Player), With<Player>>,
    pickups: Query<(&Transform, &Collider, Entity), With<Pickup>>,
    mut player_stats: ResMut<PlayerStats>
) {
    let Ok((player_transform, player_collider, mut player_self)) = player.single_mut() else {
        return;
    };

    for (pickup_transform, pickup_collider, entity) in pickups {
        let distance = pickup_transform
            .translation
            .distance(player_transform.translation);

        if distance < player_collider.radius + pickup_collider.radius {
            player_stats.gems += 1;
            commands.entity(entity).despawn();
            if player_stats.gems % 5 == 0 {
                player_self.weapon_level += 1;

            }
        }
    }
}

pub fn update_pickups(
    time: Res<Time>,
    player: Query<(&Transform, &Player), (With<Player>, Without<Pickup>)>,
    pickups: Query<(&mut Transform, &mut Pickup), With<Pickup>>,
) {
    let Ok((player_transform, player)) = player.single() else {
        return;
    };

    for (mut pickup_transform, mut pickup) in pickups {
        match pickup.state {
            PickupState::Dropped => {
                let distance = pickup_transform
                    .translation
                    .distance(player_transform.translation);

                if distance <= player.pickup_radius {
                    pickup.state = PickupState::Bounce {
                        distance_remaining: 50.0,
                    };
                }
            }
            PickupState::Bounce { distance_remaining } => {
                if distance_remaining <= 0.0 {
                    pickup.state = PickupState::Homing;
                } else {
                    let direction = -(player_transform.translation - pickup_transform.translation)
                        .normalize_or_zero();

                    pickup_transform.translation.x +=
                        PICKUP_SPEED * direction.x * time.delta_secs();
                    pickup_transform.translation.y +=
                        PICKUP_SPEED * direction.y * time.delta_secs();
                    let remaining = distance_remaining - PICKUP_SPEED * time.delta_secs();

                    pickup.state = PickupState::Bounce {
                        distance_remaining: remaining,
                    }
                }
            }
            PickupState::Homing => {
                let direction = (player_transform.translation - pickup_transform.translation)
                    .normalize_or_zero();

                pickup_transform.translation.x += PICKUP_SPEED * direction.x * time.delta_secs();
                pickup_transform.translation.y += PICKUP_SPEED * direction.y * time.delta_secs();
            }
        }
    }
}