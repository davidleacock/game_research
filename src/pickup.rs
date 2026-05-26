use bevy::prelude::*;

use crate::{components::Collider, enemy::EnemyKilled, player::Player};

#[derive(Component)]
pub struct Pickup {
    state: PickupState,
    pickup_type: PickupType,
}

pub enum PickupState {
    Dropped, // Can non-moving pickups use this?
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
    player: Query<(&Transform, &Player), With<Player>>,
    pickups: Query<(&Transform, &Collider, Entity), With<Pickup>>,
) {
    let Ok((player_transform, player_self)) = player.single() else {
        return;
    };

    for (pickup_transform, picked_collider, entity) in pickups {
        let distance = pickup_transform
            .translation
            .distance(player_transform.translation);

        if distance < player_self.pickup_radius + picked_collider.radius {
            commands.entity(entity).despawn();
        }
    }
}
