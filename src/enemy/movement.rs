pub mod move_pattern;
use super::system_label::EnemySystemLabel;
use crate::enemy::Enemy;
use crate::life_count::LifeCount;
use bevy::prelude::*;
use move_pattern::MovePattern;

pub struct EnemyMovementPlugin;

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyMoveTimer::default())
            .add_system(move_enemy_system)
            .add_system(update_enemy_velocity_system.before(EnemySystemLabel::LifeCount));
    }
}

struct EnemyMoveTimer(Timer);

impl Default for EnemyMoveTimer {
    fn default() -> Self {
        EnemyMoveTimer(Timer::from_seconds(1.0 / 40.0, true))
    }
}

fn move_enemy_system(
    time: Res<Time>,
    mut timer: ResMut<EnemyMoveTimer>,
    mut query: Query<(&Enemy, &mut Transform)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (enemy, mut transform) in query.iter_mut() {
            transform.translation += enemy.velocity;
        }
    }
}

fn update_enemy_velocity_system(
    mut query: Query<(&mut Enemy, &LifeCount, &MovePattern), Changed<LifeCount>>,
) {
    for (mut enemy, life_count, move_pattern) in query.iter_mut() {
        move_pattern
            .velocity_updater()
            .update(&mut enemy, &life_count);
    }
}
