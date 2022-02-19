use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;
use bevy::prelude::*;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .add_startup_system(setup_score_area);
    }
}

#[derive(Default)]
pub struct Score(i128);

impl Score {
    pub fn add(&mut self, value: u128) {
        self.0 += value as i128;
    }
}

fn setup_score_area(mut commands: Commands) {
    let score_area_width = WINDOW_WIDTH / 3.;
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(WINDOW_WIDTH / 2. - score_area_width / 2., 0., 1.),
            scale: Vec3::new(score_area_width, WINDOW_HEIGHT, 1.),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 153. / 255.),
            ..Default::default()
        },
        ..Default::default()
    });
}
