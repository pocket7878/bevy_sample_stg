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
    mut commands: Commands,
    player_bullet_query: Query<(Entity, &PlayerBullet, &Transform)>,
    enemy_query: Query<(Entity, &Enemy, &Transform)>,
    mut score: ResMut<Score>,
) {
    for (player_bullet_entity, _, player_bullet_transform) in player_bullet_query.iter() {
        for (enemy_entity, _, enemy_transform) in enemy_query.iter() {
            let collision = collide(
                player_bullet_transform.translation,
                player_bullet_transform.scale.truncate(),
                enemy_transform.translation,
                enemy_transform.scale.truncate(),
            );

            if collision.is_some() {
                commands.entity(enemy_entity).despawn();
                commands.entity(player_bullet_entity).despawn();
                score.add_score(100);
                break;
            }
        }
    }
}
