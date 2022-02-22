mod boss1;
use crate::in_game::life_count::LifeCount;
use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub enum MovePattern {
    Boss1,
}

// ボスの新しい座標を計算する、更新が必要であればSome, 現在の座標のままでよければNoneを返す
pub trait TranslationCalculater {
    fn calc_new_translation(&self, life_count: &LifeCount) -> Option<Vec3>;
}

impl MovePattern {
    pub fn translation_calculater(&self) -> Box<dyn TranslationCalculater> {
        match self {
            Self::Boss1 => Box::new(boss1::Boss1TranslationCalculater),
        }
    }
}
