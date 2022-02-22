use super::enemy::Bullet as EnemyBullet;
use super::game_frame::GameFrame;
use super::system_label::GameSystemLabel;
use crate::app_state::AppState;
use crate::in_game::player::Player;
use crate::in_game::player::PlayerAssets;
use crate::in_game::player::PlayerState;
use crate::in_game::scoreboard::Score;
use crate::FPS;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

const DAMAGED_INVINCIBLE_FRAME: i32 = (FPS * 2.) as i32;

pub struct PlayerStockPlugin;

impl Plugin for PlayerStockPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(hit_enemy_bullet_system)
                .with_system(decrease_damaged_invincible_frame_system)
                .before(GameSystemLabel::GameFrameUpdate),
        );
    }
}

fn hit_enemy_bullet_system(
    mut state: ResMut<State<AppState>>,
    player_assets: Res<PlayerAssets>,
    mut score: ResMut<Score>,
    enemy_bullet_query: Query<&Transform, With<EnemyBullet>>,
    mut player_query: Query<(&Transform, &mut Player, &mut Handle<Image>)>,
) {
    /*
    let (player_transform, mut player, mut sprite_handle) = player_query.single_mut();
    match player.state {
        PlayerState::Normal => {
            for enemy_bullet_transform in enemy_bullet_query.iter() {
                let collision = collide(
                    player_transform.translation,
                    player_transform.scale.truncate(),
                    enemy_bullet_transform.translation,
                    enemy_bullet_transform.scale.truncate(),
                );

                if collision.is_some() {
                    if score.on_hit_enemy_bullet() {
                        player.state = PlayerState::DamegedInvincible {
                            rest_frame: DAMAGED_INVINCIBLE_FRAME,
                        };
                        *sprite_handle = player_assets.damaged_state_handle.clone();
                    } else {
                        state.set(AppState::GameOver).unwrap();
                    }
                    // 連続被弾はしない
                    break;
                }
            }
        }
        PlayerState::DamegedInvincible { .. } => {
            // 被弾後の無敵時間中なので、被弾しない
            return;
        }
    }
    */
}

fn decrease_damaged_invincible_frame_system(
    game_frame: Res<GameFrame>,
    player_assets: Res<PlayerAssets>,
    mut player_query: Query<(&mut Player, &mut Handle<Image>), With<Player>>,
) {
    if !game_frame.is_changed() {
        return;
    }

    let (mut player, mut sprite_handle) = player_query.single_mut();
    match player.state {
        PlayerState::DamegedInvincible { ref mut rest_frame } => {
            if *rest_frame > 0 {
                *rest_frame -= 1;
            }
            if *rest_frame == 0 {
                player.state = PlayerState::Normal;
                *sprite_handle = player_assets.normal_state_handle.clone();
            }
        }
        _ => {}
    }
}
