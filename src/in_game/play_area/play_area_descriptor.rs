use bevy::prelude::*;

#[derive(Component)]
pub struct PlayAreaDescriptor {
    pub width: f32,
    pub height: f32,
    pub origin: Vec3,
}

impl PlayAreaDescriptor {
    pub fn min_x(&self) -> f32 {
        self.origin.x - self.width / 2.
    }

    pub fn max_x(&self) -> f32 {
        self.origin.x + self.width / 2.
    }

    pub fn min_y(&self) -> f32 {
        self.origin.y - self.height / 2.
    }

    pub fn max_y(&self) -> f32 {
        self.origin.y + self.height / 2.
    }

    pub fn is_outside(&self, translate: &Vec3) -> bool {
        translate.x < self.min_x()
            || translate.x > self.max_x()
            || translate.y < self.min_y()
            || translate.y > self.max_y()
    }
}
