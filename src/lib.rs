mod destroy_enemy;
mod enemy;
mod enemy_shot;
mod game_frame_count;
mod life_count;
mod play_area_descriptor;
mod player;
mod player_shot;

use bevy::{prelude::*, window::PresentMode};

pub struct GamePlugin;

const WINDOW_HEIGHT: f32 = 700.0;
const WINDOW_WIDTH: f32 = 500.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Mini Game".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(play_area_descriptor::PlayAreaDescriptor {
            min_x: -WINDOW_WIDTH / 2.0,
            max_x: WINDOW_WIDTH / 2.0,
            min_y: -WINDOW_HEIGHT / 2.0,
            max_y: WINDOW_HEIGHT / 2.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(game_frame_count::GameFrameCountPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(player_shot::PlayerShotPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(enemy_shot::EnemyShotPlugin)
        .add_startup_system(setup_camera)
        .add_system(destroy_enemy::destroy_enemy_system);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
