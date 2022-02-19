mod down_stay_left_bottom;
mod down_stay_right_bottom;
mod down_stay_up;
mod fast_down_left;
mod fast_down_right;
mod left_bottom;
mod right_bottom;

use super::Enemy;
use crate::life_count::LifeCount;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Debug, Clone, Component)]
pub enum MovePattern {
    DownStayUp,
    DownStayLeftBottom,
    DownStayRightBottom,
    FastDownLeft,
    FastDownRight,
    LeftBottom,
    RightBottom,
}

pub trait VelocityUpdater {
    fn update(&self, enemy: &mut Enemy, life_count: &LifeCount);
}

impl MovePattern {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=6) {
            0 => Self::DownStayUp,
            1 => Self::DownStayLeftBottom,
            2 => Self::DownStayRightBottom,
            3 => Self::FastDownLeft,
            4 => Self::FastDownRight,
            5 => Self::LeftBottom,
            6 => Self::RightBottom,
            _ => panic!("Unexpected strategy"),
        }
    }

    pub fn velocity_updater(&self) -> Box<dyn VelocityUpdater> {
        match self {
            Self::DownStayUp => Box::new(down_stay_up::DownStayUpPatternVelocityUpdater),
            Self::DownStayLeftBottom => {
                Box::new(down_stay_left_bottom::DownStayLeftBottomPatternVelocityUpdater)
            }
            Self::DownStayRightBottom => {
                Box::new(down_stay_right_bottom::DownStayRightBottomPatternVelocityUpdater)
            }
            Self::FastDownLeft => Box::new(fast_down_left::FastDownLeftPatternVelocityUpdater),
            Self::FastDownRight => Box::new(fast_down_right::FastDownRightPatternVelocityUpdater),
            Self::LeftBottom => Box::new(left_bottom::LeftBottomPatternVelocityUpdater),
            Self::RightBottom => Box::new(right_bottom::RightBottomPatternVelocityUpdater),
        }
    }
}
