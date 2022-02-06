use bevy::{prelude::*, window::PresentMode};

pub struct MiniGamePlugin;

const PLAYER_SIZE: f32 = 30.0;
const WINDOW_HEIGHT: f32 = 500.0;
const WINDOW_WIDTH: f32 = 500.0;
const PLAYER_X_MIN: f32 = -WINDOW_WIDTH / 2.0 + PLAYER_SIZE / 2.0;
const PLAYER_X_MAX: f32 = WINDOW_WIDTH / 2.0 - PLAYER_SIZE / 2.0;
const PLAYER_Y_MIN: f32 = -WINDOW_HEIGHT / 2.0 + PLAYER_SIZE / 2.0;
const PLAYER_Y_MAX: f32 = WINDOW_HEIGHT / 2.0 - PLAYER_SIZE / 2.0;
const GRAVITY: f32 = -1.0;

impl Plugin for MiniGamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(WindowDescriptor {
				title: "Mini Game".to_string(),
				width: WINDOW_WIDTH,
				height: WINDOW_WIDTH,
				present_mode: PresentMode::Fifo,
				resizable: false,
				..Default::default()
			})
			.add_plugins(DefaultPlugins)
			.add_startup_system(setup)
			.add_system(move_player_left_or_right_by_keyboard_system)
			.add_system(jump_player_by_keyboard_system)
			.add_system(apply_gravity_system);
	}
}

#[derive(Component)]
struct Player {
	jump_speed: f32,
	jumping: bool,
}

impl Default for Player {
	fn default() -> Self {
		Self {
			jump_speed: 0.0,
			jumping: false,
		}
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
					-(WINDOW_WIDTH / 2.0 - PLAYER_SIZE / 2.0),
					-(WINDOW_HEIGHT / 2.0 - PLAYER_SIZE / 2.0),
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
		.insert(Player::default());
}

fn move_player_left_or_right_by_keyboard_system(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Player, &mut Transform)>,
) {
	let (mut player, mut transform) = query.single_mut();
	let translation = &mut transform.translation;

	// Move Left or Right
	let move_dist = 3.0;
	if keyboard_input.pressed(KeyCode::A) {
		translation.x -= move_dist;
	}
	if keyboard_input.pressed(KeyCode::D) {
		translation.x += move_dist;
	}

	translation.x = translation
		.x
		.min(PLAYER_X_MAX)
		.max(PLAYER_X_MIN);
}

fn jump_player_by_keyboard_system(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<&mut Player>,
) {
	let mut player = query.single_mut();
	if !player.jumping && keyboard_input.just_pressed(KeyCode::Space) {
		player.jumping = true;
		player.jump_speed = 15.0;
	}
}

fn apply_gravity_system(
	mut query: Query<(&mut Player, &mut Transform)>,
) {
	let (mut player, mut transform) = query.single_mut();
	let translation = &mut transform.translation;

	if player.jumping {
		player.jump_speed += GRAVITY;
		translation.y += player.jump_speed;
	}

	if translation.y <= PLAYER_Y_MIN {
		// Player reached ground
		player.jumping = false;
		player.jump_speed = 0.0;
		translation.y = PLAYER_Y_MIN;
	} else if translation.y > PLAYER_Y_MAX {
		translation.y = PLAYER_Y_MAX;
	}
}