use super::movement::MovePattern;
use crate::app_state::AppState;
use crate::in_game::enemy::assets_holder::EnemyAssetsHolder;

use crate::in_game::enemy::Enemy;
use crate::in_game::game_frame::GameFrame;
use crate::in_game::life_count::LifeCount;
use crate::in_game::play_area::PlayAreaDescriptor;
use crate::in_game::system_label::GameSystemLabel;
use bevy::prelude::*;
use std::collections::HashMap;

/*
 * Plugin
 */
pub struct BossEnemyEmergePlugin;

impl Plugin for BossEnemyEmergePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(emerge_enemy_system.before(GameSystemLabel::GameFrameUpdate)),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup));
    }
}

/*
 * Systems
 */
fn setup(mut commands: Commands) {
    let enemy_emerge = EnemyEmerge::new();
    commands.insert_resource(enemy_emerge);
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<EnemyEmerge>();
}

#[derive(Debug)]
struct Emerge {
    initial_position: Vec3,
    hp: i32,
    bonus_score: i32,
    move_pattern: MovePattern,
}

impl Emerge {
    fn new(initial_position: Vec3, hp: i32, bonus_score: i32, move_pattern: MovePattern) -> Self {
        Self {
            initial_position,
            hp,
            bonus_score,
            move_pattern,
        }
    }
}

struct EnemyEmerge {
    emerge_map: HashMap<i128, Vec<Emerge>>,
}

impl EnemyEmerge {
    fn new() -> Self {
        let mut emerge_map = HashMap::new();
        let boss1_emerge = Emerge::new(Vec3::new(0.0, -999.9, 0.0), 100, 500, MovePattern::Boss1);
        emerge_map.insert(500i128, vec![boss1_emerge]);
        EnemyEmerge { emerge_map }
    }

    fn emerge(
        &self,
        frame: i128,
        commands: &mut Commands,
        play_area_descriptor: &PlayAreaDescriptor,
        assets_holder: &EnemyAssetsHolder,
    ) {
        let enemy_size = Vec3::new(30., 30., 30.);
        let emerge_list = self.emerge_map.get(&frame);
        if let Some(emerge_list) = emerge_list {
            for emerge in emerge_list.iter() {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: emerge.initial_position + play_area_descriptor.origin,
                            scale: enemy_size,
                            ..Default::default()
                        },
                        texture: assets_holder.pink.clone(),
                        ..Default::default()
                    })
                    .insert(Enemy {
                        hp: emerge.hp,
                        bonus_score: emerge.bonus_score,
                        is_boss_enemy: true,
                        ..Default::default()
                    })
                    .insert(LifeCount::default())
                    .insert(emerge.move_pattern.clone());
            }
        }
    }
}

fn emerge_enemy_system(
    game_frame: Res<GameFrame>,
    mut commands: Commands,
    emerger: Res<EnemyEmerge>,
    play_area_descripter: Res<PlayAreaDescriptor>,
    assets_holder: Res<EnemyAssetsHolder>,
) {
    if game_frame.is_changed() {
        emerger.emerge(
            game_frame.0,
            &mut commands,
            &play_area_descripter,
            &assets_holder,
        );
    }
}
