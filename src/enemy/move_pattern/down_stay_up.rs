use super::{Enemy, VelocityUpdater};
use crate::life_count::LifeCount;
use bevy::prelude::*;

pub struct DownStayUpPatternVelocityUpdater;

impl VelocityUpdater for DownStayUpPatternVelocityUpdater {
    fn update(&self, enemy: &mut Enemy, life_count: &LifeCount) {
        match life_count.count {
            0 => enemy.velocity = Vec3::new(0.0, -3.0, 0.0),
            40 => enemy.velocity = Vec3::new(0.0, 0.0, 0.0),
            80 => enemy.velocity = Vec3::new(0.0, 3.0, 0.0),
            _ => {}
        }
    }
}
