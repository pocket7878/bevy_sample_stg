use bevy::prelude::*;
use super::life_count;
use super::enemy::Enemy;

pub struct EnemyLifeCountTimer(Timer);

impl Default for EnemyLifeCountTimer {
	fn default() -> Self {
		EnemyLifeCountTimer(Timer::from_seconds(1.0 / 40.0, true))
	}
}

pub fn count_up_enemy_life_count_system(
	time: Res<Time>,
	mut timer: ResMut<EnemyLifeCountTimer>,
	mut query: Query<(&Enemy, &mut life_count::LifeCount)>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		for (_, mut enemy_life_count) in query.iter_mut() {
			enemy_life_count.count += 1;
		}
	}
}