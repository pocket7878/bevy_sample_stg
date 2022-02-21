pub mod move_pattern;
use super::system_label::EnemySystemLabel;
use crate::app_state::AppState;
use crate::in_game::enemy::Enemy;
use crate::in_game::game_frame::GameFrame;
use crate::in_game::life_count::LifeCount;
use crate::in_game::system_label::GameSystemLabel;
use bevy::prelude::*;
use move_pattern::MovePattern;

pub struct EnemyMovementPlugin;

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(move_enemy_system)
                .before(GameSystemLabel::GameFrameUpdate),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(update_enemy_velocity_system.before(EnemySystemLabel::LifeCount)),
        );
    }
}

fn move_enemy_system(game_frame: Res<GameFrame>, mut query: Query<(&Enemy, &mut Transform)>) {
    if game_frame.is_changed() {
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
            .update(&mut enemy, life_count);
    }
}
