use crate::in_game::{
    enemy::boss::movement::easing::easing_vec3_ease_in_out_interpolation, life_count::LifeCount,
};

use bevy::prelude::*;

use super::TranslationCalculater;

pub struct Boss1TranslationCalculater;

impl TranslationCalculater for Boss1TranslationCalculater {
    fn calc_new_translation(&self, life_count: &LifeCount) -> Option<Vec3> {
        match life_count.count {
            0..=120 => {
                let start = Vec3::new(0.0, 370., 0.0);
                let end = Vec3::new(0.0, 300.0, 0.0);
                Some(easing_vec3_ease_in_out_interpolation(
                    life_count.count as f32,
                    start,
                    end - start,
                    120.,
                ))
            }
            505..=565 => {
                let start = Vec3::new(0.0, 300., 0.0);
                let end = Vec3::new(100.0, 300.0, 0.0);
                Some(easing_vec3_ease_in_out_interpolation(
                    (life_count.count - 505) as f32,
                    start,
                    end - start,
                    60.,
                ))
            }
            945..=1065 => {
                let start = Vec3::new(100.0, 300., 0.0);
                let end = Vec3::new(-100.0, 200.0, 0.0);
                Some(easing_vec3_ease_in_out_interpolation(
                    (life_count.count - 945) as f32,
                    start,
                    end - start,
                    120.,
                ))
            }
            1450..=1570 => {
                let start = Vec3::new(-100.0, 200., 0.0);
                let end = Vec3::new(0.0, 300.0, 0.0);
                Some(easing_vec3_ease_in_out_interpolation(
                    (life_count.count - 1450) as f32,
                    start,
                    end - start,
                    120.,
                ))
            }
            _ => None,
        }
    }
}
