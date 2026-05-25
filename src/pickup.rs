use bevy::prelude::*;
use crate::enemy::EnemyKilled;

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
        }
    ));
}