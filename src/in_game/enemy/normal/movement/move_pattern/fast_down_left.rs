use super::{Enemy, VelocityUpdater};
use crate::in_game::life_count::LifeCount;
use bevy::prelude::*;

pub struct FastDownLeftPatternVelocityUpdater;

impl VelocityUpdater for FastDownLeftPatternVelocityUpdater {
    fn update(&self, enemy: &mut Enemy, life_count: &LifeCount) {
        if life_count.count == 0 {
            enemy.velocity = Vec3::new(0.0, -5.0, 0.0);
        }
        if life_count.count < 100 {
            enemy.velocity.x -= 5. / 100.;
            enemy.velocity.y -= 5. / 100.;
        }
    }
}
