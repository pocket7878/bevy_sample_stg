mod app_state;
mod destroy_enemy;
mod enemy;
mod life_count;
mod play_area;
mod player;
mod player_shot;
mod scoreboard;

use crate::play_area::PlayAreaPlugin;
use crate::scoreboard::ScoreBoardPlugin;
use app_state::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

pub const WINDOW_HEIGHT: f32 = 700.0;
pub const WINDOW_WIDTH: f32 = 700.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Mini Game".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_state(AppState::InGame)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayAreaPlugin)
        .add_plugin(ScoreBoardPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(player_shot::PlayerShotPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_startup_system(setup_camera)
        .add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(destroy_enemy::destroy_enemy_system),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
