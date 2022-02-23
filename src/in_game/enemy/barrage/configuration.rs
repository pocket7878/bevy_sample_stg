use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug, Clone)]
pub struct BarrageConfiguration {
    life_count_to_barrage_type: HashMap<i128, String>,
}

impl BarrageConfiguration {
    pub fn new() -> Self {
        Self {
            life_count_to_barrage_type: HashMap::new(),
        }
    }

    pub fn insert_barrage_type(&mut self, life_count: i128, barrage_type: &str) {
        self.life_count_to_barrage_type
            .insert(life_count, barrage_type.to_string());
    }

    pub fn get_barrage_type_for_life_count(&self, life_count: i128) -> Option<String> {
        self.life_count_to_barrage_type.get(&life_count).cloned()
    }
}
