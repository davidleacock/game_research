use crate::debug::DebugConfig;
use crate::player::PlayerStats;
use bevy::prelude::*;

mod components;
mod debug;
mod enemy;
mod map;
mod pickup;
mod player;
mod projectile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, player::setup)
        .add_systems(Startup, map::setup_world)
        .add_systems(Startup, enemy::spawn_enemies)
        .add_systems(Startup, debug::setup)
        .add_systems(Update, player::move_player)
        .add_systems(Update, enemy::move_enemies)
        .add_systems(Update, enemy::detect_collisions)
        .add_systems(Update, player::fire_weapon)
        .add_systems(Update, projectile::move_projectiles)
        .add_systems(Update, projectile::detect_projectile_collisions)
        .add_systems(Update, pickup::detect_collisions)
        .add_systems(Update, player::update_camera)
        .add_systems(Update, pickup::update_pickups)
        .add_systems(Update, debug::debug_inputs)
        .add_systems(Update, debug::draw_debug_visuals)
        .add_systems(Update, debug::draw_debug_data)
        .add_observer(pickup::on_enemy_killed)
        .insert_resource(DebugConfig {
            visuals_enabled: false,
        })
        .insert_resource(PlayerStats {
            gems: 0,
        })
        .run();
}
