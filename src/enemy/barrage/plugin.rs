use super::bullet::Bullet;
use crate::enemy::barrage::bullet::BulletType;
use crate::enemy::barrage::bulletml_runner::BulletMLRunner;
use crate::enemy::barrage::bulletml_runner::BulletMLRunnerData;
use crate::enemy::Enemy;
use crate::life_count::LifeCount;
use crate::play_area_descriptor::PlayAreaDescriptor;
use crate::player::Player;
use bevy::prelude::*;
use bevy_bulletml::BulletMLServer;
use bevy_bulletml::Runner;

pub struct EnemyBarragePlugin;

impl Plugin for EnemyBarragePlugin {
    fn build(&self, app: &mut App) {
        let mut bulletml_server = BulletMLServer::new();
        bulletml_server
            .load_file("circle", "data/barrage/circle.xml")
            .unwrap();

        app.insert_resource(BulletFrameTimer::default())
            .insert_resource(bulletml_server)
            .add_system(start_barrage_system)
            .add_system(move_enemy_bullet_system)
            .add_system(despawn_bullet_system)
            .add_system(move_enemy_bullet_system)
            .add_system(update_bullet_system);
    }
}

/*
 * Component
 */
struct BulletFrameTimer(Timer);

impl Default for BulletFrameTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0 / 60.0, true)) // 60fps
    }
}

/*
 * System
 */
fn move_enemy_bullet_system(mut query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in query.iter_mut() {
        bullet.update(&mut transform);
    }
}

fn start_barrage_system(
    bulletml_server: Res<BulletMLServer>,
    query: Query<(&Transform, &LifeCount), With<Enemy>>,
    mut commands: Commands,
) {
    let bml = bulletml_server.get("circle");
    if let Some(bml) = bml {
        for (transform, life_count) in query.iter() {
            if life_count.count == 40 {
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
    } else {
        println!("Failed to load circle.xml bulletml");
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
