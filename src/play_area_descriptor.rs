use bevy::prelude::*;

#[derive(Component)]
pub struct PlayAreaDescriptor {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl PlayAreaDescriptor {
    pub fn is_outside(&self, translate: &Vec3) -> bool {
        translate.x < self.min_x
            || translate.x > self.max_x
            || translate.y < self.min_y
            || translate.y > self.max_y
    }
}
