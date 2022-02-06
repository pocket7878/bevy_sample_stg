use bevy::{
	prelude::*,
	sprite::collide_aabb::{collide, Collision},
	window::PresentMode,
};
use rand::prelude::*;

pub struct MiniGamePlugin;

const PLAYER_SIZE: f32 = 30.0;
const WINDOW_HEIGHT: f32 = 700.0;
const WINDOW_WIDTH: f32 = 500.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct EnemyBullet;

#[derive(Component)]
struct ShotPlayerBulletTimer(Timer);

struct ShotEnemyBulletTimer(Timer);

impl Plugin for MiniGamePlugin {
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
			.insert_resource(ShotEnemyBulletTimer(Timer::from_seconds(2.0, true)))
			.add_plugins(DefaultPlugins)
			.add_startup_system(setup)
			.add_system(move_player_by_keyboard_system)
			.add_system(shot_player_bullet_by_keyboard_system)
			.add_system(repeat_player_shot_by_timer_system)
			.add_system(destroy_player_bullet_go_outside_system)
			.add_system(move_player_bullet_system)
			.add_system(destroy_enemy_system)
			.add_system(randomly_shot_enemy_bullet_system)
			.add_system(move_enemy_bullet_system)
			.add_system(destroy_enemy_bullet_go_outside_system);
	}
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
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
		.insert(Player);

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
				.insert(Enemy);
		}
	}
}

fn move_player_by_keyboard_system(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Player, &mut Transform)>,
) {
	let (_, mut transform) = query.single_mut();

	// 斜め移動も考慮して比率計算
	let move_ratio;
	if (keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D))
		&& (keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::S))
	{
		move_ratio = 0.71;
	} else {
		move_ratio = 1.0;
	}

	let move_dist = 1.0;
	if keyboard_input.pressed(KeyCode::A) {
		transform.translation.x -= move_dist * move_ratio;
	}
	if keyboard_input.pressed(KeyCode::D) {
		transform.translation.x += move_dist * move_ratio;
	}
	if keyboard_input.pressed(KeyCode::W) {
		transform.translation.y += move_dist * move_ratio;
	}
	if keyboard_input.pressed(KeyCode::S) {
		transform.translation.y -= move_dist * move_ratio;
	}

	transform.translation.x = transform
		.translation
		.x
		.min(WINDOW_WIDTH / 2.0 - transform.scale.x / 2.0)
		.max(-WINDOW_WIDTH / 2.0 + transform.scale.x / 2.0);

	transform.translation.y = transform
		.translation
		.y
		.min(WINDOW_HEIGHT / 2.0 - transform.scale.y / 2.0)
		.max(-WINDOW_HEIGHT / 2.0 + transform.scale.y / 2.0);
}

fn shot_player_bullet_by_keyboard_system(
	mut commands: Commands,
	keyboard_input: Res<Input<KeyCode>>,
	query: Query<(&Player, &Transform)>,
) {
	if keyboard_input.just_pressed(KeyCode::Space) {
		start_repeat_player_bullet_shot_timer(&mut commands);
	} else if keyboard_input.just_released(KeyCode::Space) {
		stop_repeat_player_bullet_shot_timer(&mut commands);
		return;
	} else {
		return;
	}

	let (_, transform) = query.single();
	shot_player_bullet(commands, transform);
}

fn repeat_player_shot_by_timer_system(
	commands: Commands,
	time: Res<Time>,
	mut timer: Option<ResMut<ShotPlayerBulletTimer>>,
	player_query: Query<(&Player, &Transform)>,
) {
	if let Some(ref mut timer) = timer {
		if !timer.0.tick(time.delta()).just_finished() {
			return;
		}

		let (_, transform) = player_query.single();
		shot_player_bullet(commands, transform);
	}
}

fn start_repeat_player_bullet_shot_timer(commands: &mut Commands) {
	commands.insert_resource(ShotPlayerBulletTimer(Timer::new(
		std::time::Duration::from_millis(400),
		true,
	)));
}

fn stop_repeat_player_bullet_shot_timer(commands: &mut Commands) {
	commands.remove_resource::<ShotPlayerBulletTimer>();
}

fn shot_player_bullet(mut commands: Commands, player_transform: &Transform) {
	commands
		.spawn_bundle(SpriteBundle {
			transform: Transform {
				translation: Vec3::new(
					player_transform.translation.x,
					player_transform.translation.y,
					0.0,
				),
				scale: Vec3::new(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0),
				..Default::default()
			},
			sprite: Sprite {
				color: Color::rgb(1.0, 1.0, 0.5),
				..Default::default()
			},
			..Default::default()
		})
		.insert(PlayerBullet);
}

fn move_player_bullet_system(mut query: Query<(&PlayerBullet, &mut Transform)>) {
	for (_, mut transform) in query.iter_mut() {
		transform.translation.y += 10.0;
	}
}

fn destroy_player_bullet_go_outside_system(
	mut commands: Commands,
	player_bullet_query: Query<(Entity, &PlayerBullet, &Transform)>,
) {
	for (player_bullet_entity, _, player_bullet_transform) in player_bullet_query.iter() {
		let player_bullet_bottom_y = player_bullet_transform.translation.y - PLAYER_SIZE / 2.0;
		if player_bullet_bottom_y > WINDOW_HEIGHT / 2.0 {
			commands.entity(player_bullet_entity).despawn();
		}
	}
}

fn destroy_enemy_system(
	mut commands: Commands,
	player_bullet_query: Query<(Entity, &PlayerBullet, &Transform)>,
	enemy_query: Query<(Entity, &Enemy, &Transform)>,
) {
	for (player_bullet_entity, _, player_bullet_transform) in player_bullet_query.iter() {
		for (enemy_entity, _, enemy_transform) in enemy_query.iter() {
			let collision = collide(
				player_bullet_transform.translation,
				player_bullet_transform.scale.truncate(),
				enemy_transform.translation,
				enemy_transform.scale.truncate(),
			);

			if collision.is_some() {
				commands.entity(enemy_entity).despawn();
				commands.entity(player_bullet_entity).despawn();
				break;
			}
		}
	}
}

fn randomly_shot_enemy_bullet_system(
	time: Res<Time>,
	mut timer: ResMut<ShotEnemyBulletTimer>,
	mut commands: Commands,
	enemy_query: Query<(&Enemy, &Transform)>,
) {
	if !timer.0.tick(time.delta()).just_finished() {
		return;
	}

	let mut rng = rand::thread_rng();
	let mut shot_count = rng.gen_range(0..=3);
	if shot_count == 0 {
		return;
	}
	for (_, enemy_transform) in enemy_query.iter() {
		if shot_count <= 0 {
			break;
		}

		let shot = rng.gen::<bool>();
		if shot {
			commands
				.spawn_bundle(SpriteBundle {
					transform: Transform {
						translation: Vec3::new(
							enemy_transform.translation.x,
							enemy_transform.translation.y,
							0.0,
						),
						scale: Vec3::new(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0),
						..Default::default()
					},
					sprite: Sprite {
						color: Color::rgb(0.0, 153.0 / 255.0, 51.0 / 255.0),
						..Default::default()
					},
					..Default::default()
				})
				.insert(EnemyBullet);
			shot_count -= 1;
		}
	}
}

fn move_enemy_bullet_system(mut query: Query<(&EnemyBullet, &mut Transform)>) {
	for (_, mut transform) in query.iter_mut() {
		transform.translation.y -= 3.0;
	}
}

fn destroy_enemy_bullet_go_outside_system(
	mut commands: Commands,
	enemy_bullet_query: Query<(Entity, &EnemyBullet, &Transform)>,
) {
	for (enemy_bullet_entity, _, enemy_bullet_transform) in enemy_bullet_query.iter() {
		let enemy_bullet_top_y = enemy_bullet_transform.translation.y + PLAYER_SIZE / 2.0;
		if enemy_bullet_top_y < -WINDOW_HEIGHT / 2.0 {
			commands.entity(enemy_bullet_entity).despawn();
		}
	}
}