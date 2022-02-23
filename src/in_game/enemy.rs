mod assets_holder;
mod barrage;
mod boss;
mod life_count;
mod normal;
mod system_label;

use crate::app_state::AppState;
pub use barrage::bullet::Bullet;
use barrage::EnemyBarragePlugin;
use bevy::prelude::*;
use life_count::EnemyLifeCountPlugin;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EnemyBarragePlugin)
            .add_plugin(normal::NormalEnemyEmergePlugin)
            .add_plugin(normal::NormalEnemyMovementPlugin)
            .add_plugin(boss::BossEnemyEmergePlugin)
            .add_plugin(boss::BossEnemyMovementPlugin)
            .add_plugin(EnemyLifeCountPlugin)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub velocity: Vec3,
    pub hp: i32,
    pub bonus_score: i32,
    pub is_boss_enemy: bool,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            velocity: Vec3::new(0.0, 0.0, 0.0),
            hp: 1,
            bonus_score: 100,
            is_boss_enemy: false,
        }
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

fn cleanup(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
