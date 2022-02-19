use bevy::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum EnemySystemLabel {
    LifeCount,
}
