use super::enemy::Enemy;
use super::player_shot::Bullet as PlayerBullet;
use crate::app_state::AppState;
use crate::in_game::scoreboard::Score;
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub struct DestroyEnemyPlugin;

impl Plugin for DestroyEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(destroy_enemy_system),
        );
    }
}

fn destroy_enemy_system(
    mut state: ResMut<State<AppState>>,
    mut commands: Commands,
    player_bullet_query: Query<(Entity, &PlayerBullet, &Transform)>,
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform)>,
    mut score: ResMut<Score>,
) {
    for (player_bullet_entity, _, player_bullet_transform) in player_bullet_query.iter() {
        for (enemy_entity, mut enemy, enemy_transform) in enemy_query.iter_mut() {
            let collision = collide(
                player_bullet_transform.translation,
                player_bullet_transform.scale.truncate(),
                enemy_transform.translation,
                enemy_transform.scale.truncate(),
            );

            if collision.is_some() {
                enemy.hp -= 1;
                commands.entity(player_bullet_entity).despawn();
                if enemy.hp <= 0 {
                    commands.entity(enemy_entity).despawn();
                    score.add_score(enemy.bonus_score as u128);
                    if enemy.is_boss_enemy {
                        state.set(AppState::Ending).unwrap();
                    }
                }
                break;
            }
        }
    }
}
