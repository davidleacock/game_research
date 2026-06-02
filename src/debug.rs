use crate::components::Collider;
use crate::enemy::{Enemy, EnemyState};
use crate::pickup::Pickup;
use crate::player::{Player, PlayerStats};
use bevy::prelude::*;

#[derive(Resource)]
pub struct DebugConfig {
    pub visuals_enabled: bool,
}

#[derive(Component)]
pub struct DebugInfoText;

pub fn setup(mut commands: Commands) {
    commands.spawn((DebugInfoText, Text::new("Gems: 0 Level: 1")));
}

pub fn debug_inputs(keys: Res<ButtonInput<KeyCode>>, mut config: ResMut<DebugConfig>) {
    if keys.just_pressed(KeyCode::Tab) {
        config.visuals_enabled = !config.visuals_enabled;
    }
}

pub fn draw_debug_data(
    player_stats: Res<PlayerStats>,
    player_query: Query<&Player>,
    mut text: Query<&mut Text, With<DebugInfoText>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    if let Ok(mut debug_text) = text.single_mut() {
        debug_text.0 = format!("Gems: {} Level: {}", player_stats.gems, player.weapon_level);
    }


}

pub fn draw_debug_visuals(
    config: Res<DebugConfig>,
    mut gizmos: Gizmos,
    player: Query<(&Transform, &Collider, &Player)>,
    enemies: Query<(&Transform, &Collider, &Enemy), With<Enemy>>,
    pickups: Query<(&Transform, &Collider), With<Pickup>>,
) {
    if !config.visuals_enabled {
        return;
    }

    let Ok((player_transform, player_collider, player_entity)) = player.single() else {
        return;
    };

    // players pickup radius
    gizmos.circle_2d(
        Isometry2d::from_translation(player_transform.translation.truncate()),
        player_entity.pickup_radius,
        Color::linear_rgb(1.0, 1.0, 1.0),
    );

    gizmos.arrow_2d(
        player_transform.translation.truncate(),
        player_transform.translation.truncate() + player_entity.weapon_facing * 30.0,
        Color::linear_rgb(1.0, 1.0, 1.0),
    );

    for (enemy_transform, enemy_collider, enemy) in enemies {
        match enemy.state {
            EnemyState::Roam => {
                gizmos.arrow_2d(
                    enemy_transform.translation.truncate(),
                    enemy_transform.translation.truncate() + enemy.roam_direction * 30.0,
                    Color::linear_rgb(1.0, 1.0, 1.0),
                );
            }
            EnemyState::Chase => {
                let direction = (player_transform.translation - enemy_transform.translation)
                    .truncate()
                    .normalize_or_zero();

                gizmos.arrow_2d(
                    enemy_transform.translation.truncate(),
                    enemy_transform.translation.truncate() + direction * 30.0,
                    Color::linear_rgb(1.0, 0.0, 0.0),
                );
            }
        }
    }

    for (pickup_transform, pickup_collider) in pickups {}
}
