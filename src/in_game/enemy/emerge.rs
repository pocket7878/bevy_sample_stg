use crate::app_state::AppState;
use crate::in_game::enemy::assets_holder::EnemyAssetsHolder;
use crate::in_game::enemy::barrage::configuration::BarrageConfiguration;
use crate::in_game::enemy::movement::move_pattern::MovePattern;
use crate::in_game::enemy::system_label::EnemySystemLabel;
use crate::in_game::enemy::Enemy;
use crate::in_game::life_count::LifeCount;
use crate::in_game::play_area::PlayAreaDescriptor;
use crate::FPS;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::path;

/*
 * Plugin
 */
pub struct EnemyEmergePlugin;

impl Plugin for EnemyEmergePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyEmergeFrameChangedEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(
                        count_up_enemy_emerge_frame_system.label(EnemySystemLabel::EmergeCount),
                    )
                    .with_system(emerge_enemy_system.before(EnemySystemLabel::EmergeCount)),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup));
    }
}

/*
 * Components
 */
struct EnemyEmergeFrameCount {
    frame: i128,
}

impl Default for EnemyEmergeFrameCount {
    fn default() -> Self {
        EnemyEmergeFrameCount { frame: 0 }
    }
}

struct EnemyEmergeFrameChangedEvent(i128);

// Count and notify enemy emerge frame changed.
struct EnemyEmergeTimer(Timer);

impl Default for EnemyEmergeTimer {
    fn default() -> Self {
        EnemyEmergeTimer(Timer::from_seconds(1.0 / FPS, true))
    }
}

/*
 * Systems
 */
fn setup(mut commands: Commands) {
    let mut enemy_emerge = EnemyEmerge::default();
    enemy_emerge
        .load_file("data/stage/enemy.csv")
        .expect("Faield to load enemy emerge data");
    commands.insert_resource(enemy_emerge);
    commands.insert_resource(EnemyEmergeTimer::default());
    commands.insert_resource(EnemyEmergeFrameCount::default());
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<EnemyEmerge>();
    commands.remove_resource::<EnemyEmergeTimer>();
    commands.remove_resource::<EnemyEmergeFrameCount>();
}

fn count_up_enemy_emerge_frame_system(
    mut ev_emerge_frame_changed: EventWriter<EnemyEmergeFrameChangedEvent>,
    mut count: ResMut<EnemyEmergeFrameCount>,
    time: Res<Time>,
    mut timer: ResMut<EnemyEmergeTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    count.frame += 1;
    ev_emerge_frame_changed.send(EnemyEmergeFrameChangedEvent(count.frame));
}

#[derive(Debug)]
struct Emerge {
    initial_position: Vec3,
    move_pattern: MovePattern,
    barrage_pattern: String,
    barrage_start_life_count: i128,
}

struct EnemyEmerge {
    emerge_map: HashMap<i128, Vec<Emerge>>,
}

impl Default for EnemyEmerge {
    fn default() -> Self {
        EnemyEmerge {
            emerge_map: HashMap::new(),
        }
    }
}

impl EnemyEmerge {
    fn load_file<P: AsRef<path::Path>>(&mut self, file_path: P) -> Result<(), anyhow::Error> {
        let file = File::open(file_path)?;
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        for result in rdr.records() {
            let record = result?;
            let apper_frame = record[0].parse::<i128>()?;
            let init_x = record[1].parse::<f32>()?;
            let init_y = record[2].parse::<f32>()?;
            let move_pattern_index = record[3].parse::<i32>()?;
            let barrage_pattern = record[4].to_string();
            let barrage_start_life_count = record[5].parse::<i128>()?;
            self.emerge_map
                .entry(apper_frame)
                .or_insert_with(Vec::new)
                .push(Emerge {
                    initial_position: Vec3::new(init_x, init_y, 0.0),
                    move_pattern: match move_pattern_index {
                        0 => MovePattern::DownStayUp,
                        1 => MovePattern::DownStayLeftBottom,
                        2 => MovePattern::DownStayRightBottom,
                        3 => MovePattern::FastDownLeft,
                        4 => MovePattern::FastDownRight,
                        5 => MovePattern::LeftBottom,
                        6 => MovePattern::RightBottom,
                        _ => panic!("Unsupported move type"),
                    },
                    barrage_pattern,
                    barrage_start_life_count,
                })
        }

        Ok(())
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
                        texture: assets_holder.blue.clone(),
                        ..Default::default()
                    })
                    .insert(Enemy::default())
                    .insert(LifeCount::default())
                    .insert(BarrageConfiguration {
                        barrage_type: emerge.barrage_pattern.clone(),
                        start_life_count: emerge.barrage_start_life_count,
                    })
                    .insert(emerge.move_pattern.clone());
            }
        }
    }
}

fn emerge_enemy_system(
    mut commands: Commands,
    emerger: Res<EnemyEmerge>,
    mut ev_emerge_frame_changed: EventReader<EnemyEmergeFrameChangedEvent>,
    play_area_descripter: Res<PlayAreaDescriptor>,
    assets_holder: Res<EnemyAssetsHolder>,
) {
    for ev in ev_emerge_frame_changed.iter() {
        emerger.emerge(ev.0, &mut commands, &play_area_descripter, &assets_holder);
    }
}
