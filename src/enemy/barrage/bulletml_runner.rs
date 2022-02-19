use super::bullet::{Bullet, BulletType};
use bevy::prelude::*;
use bevy_bulletml::{AppRunner, Runner, State};
use rand::prelude::*;

pub struct BulletMLRunner;
#[derive(Clone, Copy)]
pub struct BulletMLRunnerData {
    pub turn: u32,
}

impl Default for BulletMLRunnerData {
    fn default() -> Self {
        Self { turn: 0 }
    }
}

impl AppRunner<BulletMLRunnerData, Bullet> for BulletMLRunner {
    fn get_bullet_direction(&self, _: &BulletMLRunnerData, bullet: &Bullet) -> f64 {
        bullet.direction
    }

    fn get_aim_direction(
        &self,
        _: &BulletMLRunnerData,
        bullet_position: &Vec3,
        target_position: &Vec3,
    ) -> f64 {
        let rad = f64::atan2(
            (target_position.x - bullet_position.x) as f64,
            (target_position.y - bullet_position.y) as f64,
        );
        let rad_to_deg = 180.0 / std::f64::consts::PI;
        let mut result_deg = rad * rad_to_deg;
        if result_deg < 0. {
            result_deg += 360.0;
        }
        println!(
            "bullet_pos: ({}, {}), target_pos: ({}, {}), deg: {}",
            bullet_position.x, bullet_position.y, target_position.x, target_position.y, result_deg
        );

        return result_deg;
    }

    fn get_bullet_speed(&self, _: &BulletMLRunnerData, bullet: &Bullet) -> f64 {
        bullet.speed
    }

    fn get_turn(&self, data: &BulletMLRunnerData) -> u32 {
        data.turn
    }

    fn do_vanish(&mut self, _: &mut BulletMLRunnerData, bullet: &mut Bullet) {
        bullet.vanished = true;
    }

    fn get_default_speed(&self) -> f64 {
        1.
    }

    fn get_rank(&self, _: &BulletMLRunnerData) -> f64 {
        0.5
    }

    fn get_rand(&self, _: &mut BulletMLRunnerData) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }

    fn create_simple_bullet(
        &mut self,
        _: &mut BulletMLRunnerData,
        direction: f64,
        speed: f64,
        bullet_position: &Vec3,
        commands: &mut Commands,
    ) {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: bullet_position.clone(),
                    scale: Vec3::new(5., 5., 5.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Bullet {
                direction,
                speed,
                vanished: false,
            })
            .insert(BulletType::Simple);
    }

    fn create_bullet(
        &mut self,
        data: &mut BulletMLRunnerData,
        state: State,
        direction: f64,
        speed: f64,
        bullet_position: &Vec3,
        commands: &mut Commands,
    ) {
        let runner = Runner::new_from_state(BulletMLRunner, state);

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: bullet_position.clone(),
                    scale: Vec3::new(5., 5., 5.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.5, 0.7),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Bullet {
                direction,
                speed,
                vanished: false,
            })
            .insert(BulletType::WithRunner {
                data: data.clone(),
                runner: runner,
            });
    }
}
