use super::system_label::EnemySystemLabel;
use crate::app_state::AppState;
use crate::in_game::enemy::Enemy;
use crate::in_game::game_frame::GameFrame;
use crate::in_game::life_count::LifeCount;
use crate::in_game::system_label::GameSystemLabel;
use bevy::prelude::*;

pub struct EnemyLifeCountPlugin;

impl Plugin for EnemyLifeCountPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(count_up_enemy_life_count_system.label(EnemySystemLabel::LifeCount))
                .before(GameSystemLabel::GameFrameUpdate),
        );
    }
}

fn count_up_enemy_life_count_system(
    game_frame: Res<GameFrame>,
    mut query: Query<(&Enemy, &mut LifeCount)>,
) {
    if game_frame.is_changed() {
        for (_, mut enemy_life_count) in query.iter_mut() {
            enemy_life_count.count += 1;
        }
    }
}
