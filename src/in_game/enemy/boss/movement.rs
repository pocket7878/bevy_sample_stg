mod easing;
mod move_pattern;

use self::move_pattern::BossAction;
pub use self::move_pattern::MovePattern;
use crate::app_state::AppState;
use crate::in_game::enemy::barrage::BarrageStarter;
use crate::in_game::enemy::system_label::EnemySystemLabel;
use crate::in_game::enemy::Enemy;
use crate::in_game::game_frame::GameFrame;
use crate::in_game::life_count::LifeCount;
use crate::in_game::play_area::PlayAreaDescriptor;
use bevy::prelude::*;
use bevy_bulletml::BulletMLServer;

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
    mut commands: Commands,
    play_area_descriptor: Res<PlayAreaDescriptor>,
    bulletml_server: Res<BulletMLServer>,
    mut query: Query<(&mut Transform, &MovePattern, &LifeCount), With<Enemy>>,
) {
    if game_frame.is_changed() {
        for (mut transform, move_pattern, life_count) in query.iter_mut() {
            let action = move_pattern
                .action_calculater()
                .action_for_life_count(life_count);
            match action {
                BossAction::MoveTo(new_position) => {
                    let play_area_translation = new_position + play_area_descriptor.origin;
                    transform.translation = play_area_translation;
                }
                BossAction::Stay => { /* do nothing */ }
                BossAction::StartBarrrage(barrage_name) => {
                    commands
                        .start_barrage(&transform, &bulletml_server, &barrage_name)
                        .unwrap();
                }
            }
        }
    }
}
