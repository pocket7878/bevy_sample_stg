mod play_area_descriptor;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;
use bevy::prelude::*;
pub use play_area_descriptor::PlayAreaDescriptor;

pub struct PlayAreaPlugin;
const PLAY_AREA_WIDTH: f32 = WINDOW_WIDTH / 3. * 2.;
const PLAY_AREA_HEIGHT: f32 = WINDOW_HEIGHT;

impl Plugin for PlayAreaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayAreaDescriptor {
            width: PLAY_AREA_WIDTH,
            height: PLAY_AREA_HEIGHT,
            origin: Vec3::new(-WINDOW_WIDTH / 2. + PLAY_AREA_WIDTH / 2., 0., 0.),
        });
    }
}
