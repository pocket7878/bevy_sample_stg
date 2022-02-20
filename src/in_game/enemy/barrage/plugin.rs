use super::bullet::Bullet;
use crate::app_state::AppState;
use crate::in_game::enemy::barrage::bullet::BulletType;
use crate::in_game::enemy::barrage::bulletml_runner::BulletMLRunner;
use crate::in_game::enemy::barrage::bulletml_runner::BulletMLRunnerData;
use crate::in_game::enemy::barrage::configuration::BarrageConfiguration;
use crate::in_game::enemy::system_label::EnemySystemLabel;
use crate::in_game::enemy::Enemy;
use crate::in_game::life_count::LifeCount;
use crate::in_game::play_area::PlayAreaDescriptor;
use crate::in_game::player::Player;
use crate::FPS;
use bevy::prelude::*;
use bevy_bulletml::BulletMLServer;
use bevy_bulletml::Runner;

pub struct EnemyBarragePlugin;

impl Plugin for EnemyBarragePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(start_barrage_system.before(EnemySystemLabel::LifeCount))
                .with_system(move_enemy_bullet_system)
                .with_system(despawn_bullet_system)
                .with_system(move_enemy_bullet_system)
                .with_system(update_bullet_system),
        );
    }
}

/*
 * Component
 */
struct BulletFrameTimer(Timer);

impl Default for BulletFrameTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0 / FPS, true)) // 60fps
    }
}

/*
 * System
 */

fn setup(mut commands: Commands) {
    let bullet_ml_server = build_bulletml_server();
    commands.insert_resource(bullet_ml_server);
    commands.insert_resource(BulletFrameTimer::default());
}

fn move_enemy_bullet_system(mut query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in query.iter_mut() {
        bullet.update(&mut transform);
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
        if life_count.count == barrage_conf.start_life_count {
            let bml = bulletml_server.get(&barrage_conf.barrage_type);
            if let Some(bml) = bml {
                commands
                    .spawn()
                    .insert(Bullet {
                        vanished: true,
                        ..Default::default()
                    })
                    .insert(Transform {
                        translation: transform.translation,
                        ..Default::default()
                    })
                    .insert(BulletType::WithRunner {
                        data: BulletMLRunnerData::default(),
                        runner: Runner::new(BulletMLRunner, bml.clone()),
                    });
            }
        }
    }
}

fn update_bullet_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<BulletFrameTimer>,
    mut bullet_query: Query<(&mut Bullet, &mut Transform, &mut BulletType), Without<Player>>,
    ship_query: Query<(&Player, &Transform), Without<Bullet>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
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
    bulletml_server
        .load_file("circle", "data/barrage/circle.xml")
        .unwrap();
    bulletml_server
        .load_file("aim_triple", "data/barrage/aim_triple.xml")
        .unwrap();
    bulletml_server
        .load_file("none", "data/barrage/none.xml")
        .unwrap();
    bulletml_server
        .load_file("triple", "data/barrage/triple.xml")
        .unwrap();

    bulletml_server
}
