mod boss1;
use crate::in_game::life_count::LifeCount;
use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub enum MovePattern {
    Boss1,
}

pub enum BossAction {
    MoveTo(Vec3),
    Stay,
    StartBarrrage(String),
}
// ボスの新しい座標を計算する、更新が必要であればSome, 現在の座標のままでよければNoneを返す
pub trait ActionCalculater {
    fn action_for_life_count(&self, life_count: &LifeCount) -> BossAction;
}

impl MovePattern {
    pub fn action_calculater(&self) -> Box<dyn ActionCalculater> {
        match self {
            Self::Boss1 => Box::new(boss1::Boss1ActionCalculater::new()),
        }
    }
}
