use crate::app_state::AppState;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;
use bevy::prelude::*;

const DEFAULT_PLAYER_STOCK: i32 = 3;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(display_score_system)
                    .with_system(display_player_stock_system),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup));
    }
}

pub struct Score {
    pub score: i128,
    player_stock: i32,
}

impl Default for Score {
    fn default() -> Self {
        Score {
            score: 0,
            player_stock: DEFAULT_PLAYER_STOCK,
        }
    }
}

impl Score {
    pub fn add_score(&mut self, score_diff: u128) {
        self.score += score_diff as i128;
    }

    pub fn on_hit_enemy_bullet(&mut self) -> bool {
        if self.player_stock > 0 {
            self.player_stock -= 1;
        }

        self.player_stock > 0
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct PlayerStockText;

struct ScoreBoardEntities {
    entities: Vec<Entity>,
}

fn setup(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    commands.insert_resource(Score::default());
    setup_score_area(&mut commands, &mut asset_server)
}

fn setup_score_area(commands: &mut Commands, asset_server: &mut AssetServer) {
    let mut score_board_entities = vec![];

    let score_area_width = WINDOW_WIDTH / 3.;
    let score_area_translation = Vec3::new(WINDOW_WIDTH / 2. - score_area_width / 2., 0., 1.);

    score_board_entities.push(spawn_scoreboard_background(
        commands,
        score_area_translation,
        score_area_width,
    ));
    score_board_entities.append(&mut spawn_score_texts(commands, asset_server));
    score_board_entities.append(&mut spawn_stock_texts(commands, asset_server));

    commands.insert_resource(ScoreBoardEntities {
        entities: score_board_entities,
    });
}

fn cleanup(mut commands: Commands, score_board_entities: Res<ScoreBoardEntities>) {
    for e in score_board_entities.entities.iter() {
        commands.entity(*e).despawn_recursive();
    }
}

fn display_score_system(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("{}", score.score);
}

fn display_player_stock_system(
    score: Res<Score>,
    mut query: Query<&mut Text, With<PlayerStockText>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("{}", score.player_stock);
}

/*
 * Utility
 */
fn spawn_scoreboard_background(
    commands: &mut Commands,
    score_area_translation: Vec3,
    score_area_width: f32,
) -> Entity {
    let entity = commands
        .spawn_bundle(SpriteBundle {
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
        })
        .id();

    entity
}

fn spawn_score_texts(commands: &mut Commands, asset_server: &AssetServer) -> Vec<Entity> {
    let mut result = vec![];

    // Display Score
    result.push(
        commands
            .spawn_bundle(TextBundle {
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
            })
            .id(),
    );

    result.push(
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
            .insert(ScoreText)
            .id(),
    );

    result
}

fn spawn_stock_texts(commands: &mut Commands, asset_server: &AssetServer) -> Vec<Entity> {
    let mut result = vec![];

    result.push(
        commands
            .spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Stock".to_string(),
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
                        top: Val::Px(130.),
                        left: Val::Px(WINDOW_WIDTH / 3. * 2. + 50.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .id(),
    );

    result.push(
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
                        top: Val::Px(154.),
                        left: Val::Px(WINDOW_WIDTH / 3. * 2. + 50.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlayerStockText)
            .id(),
    );

    result
}
