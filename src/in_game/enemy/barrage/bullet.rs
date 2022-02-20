use crate::in_game::enemy::barrage::bulletml_runner::BulletMLRunner;
use crate::in_game::enemy::barrage::bulletml_runner::BulletMLRunnerData;
use bevy::prelude::*;
use bevy_bulletml::Runner;

/*
 * Component
 */
#[derive(Component)]
pub struct Bullet {
    pub direction: f64,
    pub speed: f64,
    pub vanished: bool,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            direction: 0.,
            speed: 0.,
            vanished: false,
        }
    }
}

impl Bullet {
    pub fn update(&self, transform: &mut Transform) {
        transform.translation.x +=
            (f64::sin(self.direction * std::f64::consts::PI / 180.) * self.speed) as f32;
        transform.translation.y +=
            (f64::cos(self.direction * std::f64::consts::PI / 180.) * self.speed) as f32;
    }
}

#[derive(Component)]
pub enum BulletType {
    Simple,
    WithRunner {
        data: BulletMLRunnerData,
        runner: Runner<BulletMLRunner>,
    },
}
