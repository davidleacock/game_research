use bevy::prelude::*;

mod components;
mod enemy;
mod map;
mod player;
mod projectile;
mod pickup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, player::setup)
        .add_systems(Startup, map::setup_world)
        .add_systems(Startup, enemy::spawn_enemies)
        .add_systems(Update, player::move_player)
        .add_systems(Update, enemy::move_enemies)
        .add_systems(Update, enemy::detect_collisions)
        .add_systems(Update, player::fire_weapon)
        .add_systems(Update, projectile::move_projectiles)
        .add_systems(Update, projectile::detect_projectile_collisions)
        .add_systems(Update, enemy::debug_inputs)
        .add_systems(Update, player::update_camera)
        .add_observer(pickup::on_enemy_killed)
        .run();
}
