mod play_area_descriptor;
mod game_frame_count;
mod player;
mod player_shot;
mod life_count;
mod enemy;
mod enemy_life_counter;
mod enemy_shot;
mod destroy_enemy;

use bevy::{
	prelude::*,
	window::PresentMode,
};
use enemy::Enemy;

pub struct GamePlugin;

const PLAYER_SIZE: f32 = 30.0;
const WINDOW_HEIGHT: f32 = 700.0;
const WINDOW_WIDTH: f32 = 500.0;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(WindowDescriptor {
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
			.insert_resource(game_frame_count::GameFrameCount::default())
			.insert_resource(game_frame_count::GameFrameCountTimer::default())
			.insert_resource(enemy_shot::ShotBulletTimer::default())
			.insert_resource(enemy_life_counter::EnemyLifeCountTimer::default())
			.add_plugins(DefaultPlugins)
			.add_startup_system(setup)
			.add_system(game_frame_count::count_up_game_frame_system)
			.add_system(enemy_life_counter::count_up_enemy_life_count_system)
			.add_system(player::move_player_by_keyboard_system)
			.add_system(player_shot::shot_player_bullet_by_keyboard_system)
			.add_system(player_shot::repeat_player_shot_by_timer_system)
			.add_system(player_shot::destroy_player_bullet_go_outside_system)
			.add_system(player_shot::move_player_bullet_system)
			.add_system(destroy_enemy::destroy_enemy_system)
			.add_system(enemy_shot::randomly_shot_enemy_bullet_system)
			.add_system(enemy_shot::move_enemy_bullet_system)
			.add_system(enemy_shot::destroy_enemy_bullet_go_outside_system);
	}
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());

	// Load assets
	let rocket_asset_handle: Handle<Image> = asset_server.load("images/rocket.png");
	let alien_handles: Vec<Handle<Image>> = vec![
		asset_server.load("images/blue_alien.png"),
		asset_server.load("images/pink_alien.png"),
		asset_server.load("images/purple_alien.png"),
		asset_server.load("images/yellow_alien.png"),
	];
	// Player
	commands
		.spawn_bundle(SpriteBundle {
			transform: Transform {
				translation: Vec3::new(
					0.0 - PLAYER_SIZE / 2.0,
					-(WINDOW_HEIGHT / 2.0 - PLAYER_SIZE / 2.0) + PLAYER_SIZE * 3.0,
					0.0,
				),
				scale: Vec3::new(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE),
				..Default::default()
			},
			sprite: Sprite {
				custom_size: Some(Vec2::new(1.0, 1.0)),
				..Default::default()
			},
			texture: rocket_asset_handle.into(),
			..Default::default()
		})
		.insert(player::Player);

	// Enemy
	let enemy_rows = 4;
	let enemy_columns = 6;
	let enemy_spacing = 21.0;
	let enemy_size = Vec3::new(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE);
	let enemies_width = enemy_columns as f32 * (enemy_size.x + enemy_spacing) - enemy_spacing;
	// center the bricks and move them up a bit
	let enemies_offset = Vec3::new(-(enemies_width - enemy_size.x) / 2.0, 100.0, 0.0);
	for row in 0..enemy_rows {
		let y_position = row as f32 * (enemy_size.y + enemy_spacing);
		for column in 0..enemy_columns {
			let brick_position = Vec3::new(
				column as f32 * (enemy_size.x + enemy_spacing),
				y_position,
				0.0,
			) + enemies_offset;

			commands
				.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						custom_size: Some(Vec2::new(1.0, 1.0)),
						..Default::default()
					},
					transform: Transform {
						translation: brick_position,
						scale: enemy_size,
						..Default::default()
					},
					texture: alien_handles[row].clone().into(),
					..Default::default()
				})
				.insert(Enemy)
				.insert(life_count::LifeCount::default());
		}
	}
}
