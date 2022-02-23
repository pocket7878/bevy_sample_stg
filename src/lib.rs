mod app_state;
mod ending;
mod game_over;
mod in_game;
mod menu;

use app_state::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

pub const WINDOW_HEIGHT: f32 = 700.0;
pub const WINDOW_WIDTH: f32 = 700.0;
pub const FPS: f32 = 60.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Mini Game".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_state(AppState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(in_game::InGamePlugin)
        .add_plugin(game_over::GameOverPlugin)
        .add_plugin(ending::EndingPlugin)
        .add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
