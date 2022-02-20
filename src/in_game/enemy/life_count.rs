use super::system_label::EnemySystemLabel;
use crate::app_state::AppState;
use crate::in_game::enemy::Enemy;
use crate::in_game::life_count::LifeCount;
use crate::FPS;
use bevy::prelude::*;

pub struct EnemyLifeCountPlugin;

impl Plugin for EnemyLifeCountPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(
                    count_up_enemy_life_count_system.label(EnemySystemLabel::LifeCount),
                ),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup));
    }
}

struct EnemyLifeCountTimer(Timer);

impl Default for EnemyLifeCountTimer {
    fn default() -> Self {
        EnemyLifeCountTimer(Timer::from_seconds(1.0 / FPS, true))
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(EnemyLifeCountTimer::default())
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<EnemyLifeCountTimer>()
}

fn count_up_enemy_life_count_system(
    time: Res<Time>,
    mut timer: ResMut<EnemyLifeCountTimer>,
    mut query: Query<(&Enemy, &mut LifeCount)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (_, mut enemy_life_count) in query.iter_mut() {
            enemy_life_count.count += 1;
        }
    }
}
