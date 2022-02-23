mod easing;
mod move_pattern;



pub use self::move_pattern::MovePattern;
use crate::app_state::AppState;
use crate::in_game::enemy::system_label::EnemySystemLabel;
use crate::in_game::enemy::Enemy;
use crate::in_game::game_frame::GameFrame;
use crate::in_game::life_count::LifeCount;
use crate::in_game::play_area::PlayAreaDescriptor;
use bevy::prelude::*;

pub struct BossEnemyMovementPlugin;

impl Plugin for BossEnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(move_boss_system.before(EnemySystemLabel::LifeCount)),
        );
    }
}

fn move_boss_system(
    game_frame: Res<GameFrame>,
    play_area_descriptor: Res<PlayAreaDescriptor>,
    mut query: Query<(&mut Transform, &MovePattern, &LifeCount), With<Enemy>>,
) {
    if game_frame.is_changed() {
        for (mut transform, move_pattern, life_count) in query.iter_mut() {
            // もし座標の更新が必要であれば、ゲームのプレイングエリアの座標に変換してから反映したい
            let raw_new_translation = move_pattern
                .translation_calculater()
                .calc_new_translation(life_count);
            if let Some(raw_new_translation) = raw_new_translation {
                let new_translation = raw_new_translation + play_area_descriptor.origin;
                transform.translation = new_translation;
            }
        }
    }
}
