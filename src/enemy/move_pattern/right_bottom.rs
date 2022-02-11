use super::{Enemy, VelocityUpdater};
use crate::life_count::LifeCount;
use bevy::prelude::*;

pub struct RightBottomPatternVelocityUpdater;

impl VelocityUpdater for RightBottomPatternVelocityUpdater {
	fn update(&self, enemy: &mut Enemy, life_count: &LifeCount) {
		if life_count.count == 0 {
			enemy.velocity = Vec3::new(1.0, -2.0, 0.0);
		}
	}
}
