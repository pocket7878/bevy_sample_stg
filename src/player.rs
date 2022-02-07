use bevy::prelude::*;
use super::play_area_descriptor::PlayAreaDescriptor;

#[derive(Component)]
pub struct Player;

pub fn move_player_by_keyboard_system(
	play_area: Res<PlayAreaDescriptor>,
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
		.min(play_area.max_x - transform.scale.x)
		.max(play_area.min_x + transform.scale.y);

	transform.translation.y = transform
		.translation
		.y
		.min(play_area.max_y - transform.scale.y / 2.0)
		.max(play_area.min_y + transform.scale.y / 2.0);
}