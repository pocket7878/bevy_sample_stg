use crate::in_game::{
    enemy::boss::movement::easing::easing_vec3_ease_in_out_interpolation, life_count::LifeCount,
};

use bevy::prelude::*;

use super::{ActionCalculater, BossAction};

pub struct Boss1ActionCalculater;

impl ActionCalculater for Boss1ActionCalculater {
    fn action_for_life_count(&self, life_count: &LifeCount) -> BossAction {
        match life_count.count {
            0..=120 => {
                let start = Vec3::new(0.0, 370., 0.0);
                let end = Vec3::new(0.0, 300.0, 0.0);
                BossAction::MoveTo(easing_vec3_ease_in_out_interpolation(
                    life_count.count as f32,
                    start,
                    end - start,
                    120.,
                ))
            }
            121 => BossAction::StartBarrrage("boss1_first_wave".to_string()),
            505..=565 => {
                let start = Vec3::new(0.0, 300., 0.0);
                let end = Vec3::new(100.0, 300.0, 0.0);
                BossAction::MoveTo(easing_vec3_ease_in_out_interpolation(
                    (life_count.count - 505) as f32,
                    start,
                    end - start,
                    60.,
                ))
            }
            566 => BossAction::StartBarrrage("boss1_second_wave".to_string()),
            945..=1065 => {
                let start = Vec3::new(100.0, 300., 0.0);
                let end = Vec3::new(-100.0, 200.0, 0.0);
                BossAction::MoveTo(easing_vec3_ease_in_out_interpolation(
                    (life_count.count - 945) as f32,
                    start,
                    end - start,
                    120.,
                ))
            }
            1066 => BossAction::StartBarrrage("boss1_first_wave".to_string()),
            1450..=1570 => {
                let start = Vec3::new(-100.0, 200., 0.0);
                let end = Vec3::new(0.0, 300.0, 0.0);
                BossAction::MoveTo(easing_vec3_ease_in_out_interpolation(
                    (life_count.count - 1450) as f32,
                    start,
                    end - start,
                    120.,
                ))
            }
            1571 => BossAction::StartBarrrage("boss1_second_wave".to_string()),
            _ => BossAction::Stay,
        }
    }
}
