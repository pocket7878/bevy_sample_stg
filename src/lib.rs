mod app_state;
mod in_game;

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
        .add_plugin(in_game::play_area::PlayAreaPlugin)
        .add_plugin(in_game::scoreboard::ScoreBoardPlugin)
        .add_plugin(in_game::player::PlayerPlugin)
        .add_plugin(in_game::player_shot::PlayerShotPlugin)
        .add_plugin(in_game::enemy::EnemyPlugin)
        .add_plugin(in_game::destroy_enemy::DestroyEnemyPlugin)
        .add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
