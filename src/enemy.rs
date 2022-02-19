mod assets_holder;
mod barrage;
mod emerge;
mod movement;
mod system_label;

use super::life_count::LifeCount;
use barrage::EnemyBarragePlugin;
use bevy::prelude::*;
use emerge::EnemyEmergePlugin;
use movement::EnemyMovementPlugin;
use system_label::EnemySystemLabel;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyLifeCountTimer::default())
            .add_plugin(EnemyBarragePlugin)
            .add_plugin(EnemyEmergePlugin)
            .add_plugin(EnemyMovementPlugin)
            .add_startup_system(setup)
            .add_system(count_up_enemy_life_count_system.label(EnemySystemLabel::LifeCount));
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
