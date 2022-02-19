use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BarrageConfiguration {
    pub barrage_type: String,
    pub start_life_count: i128,
}
