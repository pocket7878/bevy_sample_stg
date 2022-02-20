mod assets_holder;
mod barrage;
mod emerge;
mod life_count;
mod movement;
mod system_label;

use crate::app_state::AppState;
use barrage::EnemyBarragePlugin;
use bevy::prelude::*;
use emerge::EnemyEmergePlugin;
use life_count::EnemyLifeCountPlugin;
use movement::EnemyMovementPlugin;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EnemyBarragePlugin)
            .add_plugin(EnemyEmergePlugin)
            .add_plugin(EnemyMovementPlugin)
            .add_plugin(EnemyLifeCountPlugin)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup));
    }
}

#[derive(Component, Default)]
pub struct Enemy {
    pub velocity: Vec3,
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
