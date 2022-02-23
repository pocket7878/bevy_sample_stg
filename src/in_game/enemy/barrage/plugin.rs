use super::bullet::Bullet;
use super::BarrageStarter;
use crate::app_state::AppState;
use crate::in_game::enemy::barrage::bullet::BulletType;
use crate::in_game::enemy::barrage::bulletml_runner::BulletMLRunner;
use crate::in_game::enemy::barrage::bulletml_runner::BulletMLRunnerData;
use crate::in_game::enemy::barrage::configuration::BarrageConfiguration;
use crate::in_game::enemy::system_label::EnemySystemLabel;
use crate::in_game::enemy::Enemy;
use crate::in_game::game_frame::GameFrame;
use crate::in_game::life_count::LifeCount;
use crate::in_game::play_area::PlayAreaDescriptor;
use crate::in_game::player::Player;
use bevy::prelude::*;
use bevy_bulletml::BulletMLServer;
use bevy_bulletml::Runner;
use std::fs;
use std::path::PathBuf;

pub struct EnemyBarragePlugin;

impl Plugin for EnemyBarragePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(start_barrage_system.before(EnemySystemLabel::LifeCount))
                    .with_system(move_enemy_bullet_system)
                    .with_system(despawn_bullet_system)
                    .with_system(move_enemy_bullet_system)
                    .with_system(update_bullet_system),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup));
    }
}

/*
 * System
 */

fn setup(mut commands: Commands) {
    let bullet_ml_server = build_bulletml_server();
    commands.insert_resource(bullet_ml_server);
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Bullet>>) {
    commands.remove_resource::<BulletMLServer>();
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn move_enemy_bullet_system(
    game_frame: Res<GameFrame>,
    mut query: Query<(&Bullet, &mut Transform)>,
) {
    if game_frame.is_changed() && game_frame.0 > 0 {
        for (bullet, mut transform) in query.iter_mut() {
            bullet.update(&mut transform);
        }
    }
}

fn start_barrage_system(
    bulletml_server: Res<BulletMLServer>,
    query: Query<
        (&Transform, &LifeCount, &BarrageConfiguration),
        (With<Enemy>, Changed<LifeCount>),
    >,
    mut commands: Commands,
) {
    for (transform, life_count, barrage_conf) in query.iter() {
        if let Some(barrage_type_name) =
            barrage_conf.get_barrage_type_for_life_count(life_count.count)
        {
            commands
                .start_barrage(transform, &bulletml_server, &barrage_type_name)
                .unwrap();
        }
    }
}

fn update_bullet_system(
    game_frame: Res<GameFrame>,
    mut commands: Commands,
    mut bullet_query: Query<(&mut Bullet, &mut Transform, &mut BulletType), Without<Player>>,
    ship_query: Query<(&Player, &Transform), Without<Bullet>>,
) {
    if !game_frame.is_changed() {
        return;
    }

    let (_, player_transform) = ship_query.single();
    for (mut bullet, mut transform, mut bullet_type) in &mut bullet_query.iter_mut() {
        match *bullet_type {
            BulletType::Simple => {
                bullet.update(&mut transform);
            }
            BulletType::WithRunner {
                ref mut data,
                ref mut runner,
            } => {
                bullet.update(&mut transform);
                runner.run(
                    data,
                    &mut bullet,
                    &transform.translation,
                    &player_transform.translation,
                    &mut commands,
                );
                data.turn += 1
            }
        }
    }
}

fn despawn_bullet_system(
    play_area: Res<PlayAreaDescriptor>,
    mut commands: Commands,
    query: Query<(Entity, &Bullet, &Transform, &BulletType)>,
) {
    for (entity, bullet, transform, bullet_type) in query.iter() {
        match *bullet_type {
            BulletType::Simple => {
                if play_area.is_outside(&transform.translation) {
                    commands.entity(entity).despawn();
                }
            }
            BulletType::WithRunner { ref runner, .. } => {
                if (play_area.is_outside(&transform.translation) || bullet.vanished)
                    && runner.is_end()
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

/*
 * Utils
 */
fn build_bulletml_server() -> BulletMLServer {
    let mut bulletml_server = BulletMLServer::new();
    let barrage_dir_entries = fs::read_dir("data/barrage").unwrap();
    let bulletml_file_paths: Vec<PathBuf> = barrage_dir_entries
        .filter_map(|e| {
            let ent = e.unwrap();
            if !is_bulletml_file_entry(&ent) {
                None
            } else {
                Some(ent.path())
            }
        })
        .collect();

    for p in bulletml_file_paths {
        // Use filestem as a key name.
        let key = p.file_stem().unwrap().to_str().unwrap().to_string();
        bulletml_server.load_file(&key, p).unwrap();
    }

    bulletml_server
}

fn is_bulletml_file_entry(entry: &fs::DirEntry) -> bool {
    let file_type = entry.file_type().unwrap();
    // Assume all XML files are BulletML file.
    file_type.is_file() && entry.file_name().to_str().unwrap().ends_with(".xml")
}
