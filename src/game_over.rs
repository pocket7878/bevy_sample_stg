use crate::{app_state::AppState, in_game::scoreboard::Score};

use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::GameOver)
                .with_system(setup_title)
                .with_system(setup_menu),
        )
        .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(menu_system))
        .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(cleanup));
    }
}

struct TitleData {
    title_entity: Entity,
}

struct MenuData {
    button_entity: Entity,
}

#[derive(Component)]
struct ScoreText;

fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    let title_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(30.)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "GAME OVER".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/x8y12pxTheStrongGamer.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1., 1., 1.),
                        },
                    }],
                    ..Default::default()
                },
                style: Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("Score: {}", score.score),
                            style: TextStyle {
                                font: asset_server.load("fonts/x8y12pxTheStrongGamer.ttf"),
                                font_size: 24.0,
                                color: Color::rgb(1., 1., 1.),
                            },
                        }],
                        ..Default::default()
                    },
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScoreText);
        })
        .id();
    commands.insert_resource(TitleData { title_entity });
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Play Again",
                    TextStyle {
                        font: asset_server.load("fonts/x8y12pxTheStrongGamer.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu_system(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup(mut commands: Commands, title_data: Res<TitleData>, menu_data: Res<MenuData>) {
    commands.entity(title_data.title_entity).despawn_recursive();
    commands.entity(menu_data.button_entity).despawn_recursive();
    commands.remove_resource::<Score>();
}
