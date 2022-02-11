use bevy::prelude::*;

pub struct GameFrameCountPlugin;

impl Plugin for GameFrameCountPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(GameFrameCount::default())
			.insert_resource(GameFrameCountTimer::default())
			.add_system(count_up_game_frame_system);
	}
}

// Count game frame
#[derive(Default)]
pub struct GameFrameCount {
	pub count: i128,
}

pub struct GameFrameCountTimer(Timer);

impl Default for GameFrameCountTimer {
	fn default() -> Self {
		GameFrameCountTimer(Timer::from_seconds(1.0 / 40.0, true))
	}
}

// Count up game frame
fn count_up_game_frame_system(
	mut game_frame_count: ResMut<GameFrameCount>,
	mut game_frame_count_timer: ResMut<GameFrameCountTimer>,
	time: Res<Time>,
) {
	if game_frame_count_timer.0.tick(time.delta()).just_finished() {
		game_frame_count.count += 1;
	}
}