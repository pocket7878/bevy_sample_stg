use bevy::prelude::*;
use super::play_area_descriptor::PlayAreaDescriptor;
use super::player::{Player};

const BULLET_SIZE: f32 = 15.0;

/*
 * Component
 */
#[derive(Component)]
pub struct Bullet;

pub struct ShotPlayerBulletTimer(Timer);

/*
 * System
 */
pub fn shot_player_bullet_by_keyboard_system(
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

pub fn repeat_player_shot_by_timer_system(
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

pub fn move_player_bullet_system(mut query: Query<(&Bullet, &mut Transform)>) {
	for (_, mut transform) in query.iter_mut() {
		transform.translation.y += 10.0;
	}
}

pub fn destroy_player_bullet_go_outside_system(
	play_area: Res<PlayAreaDescriptor>,
	mut commands: Commands,
	player_bullet_query: Query<(Entity, &Bullet, &Transform)>,
) {
	for (player_bullet_entity, _, player_bullet_transform) in player_bullet_query.iter() {
		let player_bullet_bottom_y = player_bullet_transform.translation.y - BULLET_SIZE;
		if player_bullet_bottom_y >  play_area.max_y {
			commands.entity(player_bullet_entity).despawn();
		}
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
				scale: Vec3::new(BULLET_SIZE, BULLET_SIZE, BULLET_SIZE),
				..Default::default()
			},
			sprite: Sprite {
				color: Color::rgb(1.0, 1.0, 0.5),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Bullet);
}