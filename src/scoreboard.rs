use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;
use bevy::prelude::*;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .add_startup_system(setup_score_area)
            .add_system(display_score_system);
    }
}

#[derive(Default)]
pub struct Score {
    score: i128,
}

impl Score {
    pub fn add_score(&mut self, score_diff: u128) {
        self.score += score_diff as i128;
    }
}

#[derive(Component)]
struct ScoreText;

fn setup_score_area(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let score_area_width = WINDOW_WIDTH / 3.;
    let score_area_translation = Vec3::new(WINDOW_WIDTH / 2. - score_area_width / 2., 0., 1.);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: score_area_translation,
            scale: Vec3::new(score_area_width, WINDOW_HEIGHT, 1.),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 153. / 255.),
            ..Default::default()
        },
        ..Default::default()
    });

    // Display Score
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Score".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/x8y12pxTheStrongGamer.ttf"),
                    font_size: 24.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            }],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(50.),
                left: Val::Px(WINDOW_WIDTH / 3. * 2. + 50.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/x8y12pxTheStrongGamer.ttf"),
                        font_size: 24.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                }],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(80.),
                    left: Val::Px(WINDOW_WIDTH / 3. * 2. + 50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreText);
}

fn display_score_system(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("{}", score.score);
}
