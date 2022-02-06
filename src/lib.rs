use bevy::{
	prelude::*,
	sprite::collide_aabb::{collide, Collision},
	window::PresentMode,
};

pub struct MiniGamePlugin;

const PLAYER_SIZE: f32 = 30.0;
const WINDOW_HEIGHT: f32 = 700.0;
const WINDOW_WIDTH: f32 = 500.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
enum Collider {
	Enemy,
}

#[derive(Component)]
struct ShotBulletTimer(Timer);

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
			.add_plugins(DefaultPlugins)
			.add_startup_system(setup)
			.add_system(move_player_by_keyboard_system)
			.add_system(shot_bullet_by_keyboard_system)
			.add_system(repeat_shot_by_timer_system)
			.add_system(move_bullet_system);
	}
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());

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
				color: Color::rgb(1.0, 0.5, 0.5),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Player);
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

fn shot_bullet_by_keyboard_system(
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

fn repeat_shot_by_timer_system(
	commands: Commands,
	time: Res<Time>,
	mut timer: Option<ResMut<ShotBulletTimer>>,
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
	commands.insert_resource(ShotBulletTimer(Timer::new(
		std::time::Duration::from_millis(400),
		true,
	)));
}

fn stop_repeat_player_bullet_shot_timer(commands: &mut Commands) {
	commands.remove_resource::<ShotBulletTimer>();
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

fn move_bullet_system(mut query: Query<(&PlayerBullet, &mut Transform)>) {
	for (_, mut transform) in query.iter_mut() {
		transform.translation.y += 10.0;
	}
}
