mod assets_holder;
mod barrage;
mod emerge;
mod move_pattern;

use super::life_count::LifeCount;
use crate::enemy::emerge::EnemyEmergePlugin;
use barrage::EnemyBarragePlugin;
use bevy::prelude::*;
use move_pattern::MovePattern;

const ENEMY_SIZE: f32 = 30.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyLifeCountTimer::default())
            .insert_resource(EnemyMoveTimer::default())
            .add_plugin(EnemyBarragePlugin)
            .add_plugin(EnemyEmergePlugin)
            .add_startup_system(setup)
            .add_system(count_up_enemy_life_count_system)
            .add_system(move_enemy_system)
            .add_system(update_enemy_velocity_system);
    }
}

#[derive(Component, Default)]
pub struct Enemy {
    pub velocity: Vec3,
}

struct EnemyLifeCountTimer(Timer);

impl Default for EnemyLifeCountTimer {
    fn default() -> Self {
        EnemyLifeCountTimer(Timer::from_seconds(1.0 / 40.0, true))
    }
}

struct EnemyMoveTimer(Timer);

impl Default for EnemyMoveTimer {
    fn default() -> Self {
        EnemyMoveTimer(Timer::from_seconds(1.0 / 40.0, true))
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Load & store assets
    let assets_holder = assets_holder::EnemyAssetsHolder {
        blue: asset_server.load("images/blue_alien.png"),
        pink: asset_server.load("images/pink_alien.png"),
        purple: asset_server.load("images/purple_alien.png"),
        yellow: asset_server.load("images/yellow_alien.png"),
    };
    commands.insert_resource(assets_holder);
}

fn count_up_enemy_life_count_system(
    time: Res<Time>,
    mut timer: ResMut<EnemyLifeCountTimer>,
    mut query: Query<(&Enemy, &mut LifeCount)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (_, mut enemy_life_count) in query.iter_mut() {
            enemy_life_count.count += 1;
        }
    }
}

fn move_enemy_system(
    time: Res<Time>,
    mut timer: ResMut<EnemyMoveTimer>,
    mut query: Query<(&Enemy, &mut Transform)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (enemy, mut transform) in query.iter_mut() {
            transform.translation += enemy.velocity;
        }
    }
}

fn update_enemy_velocity_system(mut query: Query<(&mut Enemy, &LifeCount, &MovePattern)>) {
    for (mut enemy, life_count, move_pattern) in query.iter_mut() {
        move_pattern
            .velocity_updater()
            .update(&mut enemy, &life_count);
    }
}
