use super::movement::MovePattern;
use crate::app_state::AppState;
use crate::in_game::enemy::assets_holder::EnemyAssetsHolder;
use crate::in_game::enemy::barrage::configuration::BarrageConfiguration;
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
    move_pattern: MovePattern,
    barrage_configuration: BarrageConfiguration,
}

impl Emerge {
    fn new(initial_position: Vec3, move_pattern: MovePattern) -> Self {
        Self {
            initial_position,
            move_pattern,
            barrage_configuration: BarrageConfiguration::new(),
        }
    }

    fn set_barrage_pattern_for_life_count(&mut self, life_count: i128, barrage_pattern: &str) {
        self.barrage_configuration
            .insert_barrage_type(life_count, barrage_pattern);
    }
}

struct EnemyEmerge {
    emerge_map: HashMap<i128, Vec<Emerge>>,
}

impl EnemyEmerge {
    fn new() -> Self {
        let mut emerge_map = HashMap::new();
        let mut boss1_emerge = Emerge::new(Vec3::new(0.0, -999.9, 0.0), MovePattern::Boss1);
        boss1_emerge.set_barrage_pattern_for_life_count(120, "boss1_first_wave");
        boss1_emerge.set_barrage_pattern_for_life_count(565, "boss1_second_wave");
        boss1_emerge.set_barrage_pattern_for_life_count(1065, "boss1_first_wave");
        boss1_emerge.set_barrage_pattern_for_life_count(1570, "boss1_second_wave");
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
                    .insert(Enemy::default())
                    .insert(LifeCount::default())
                    .insert(emerge.barrage_configuration.clone())
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
