use crate::app_state::AppState;

use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Menu)
                .with_system(setup_camera)
                .with_system(setup_title)
                .with_system(setup_menu),
        )
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu_system))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu));
    }
}

struct TitleData {
    title_entity: Entity,
}

struct MenuData {
    button_entity: Entity,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(30.)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: Rect {
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
                        value: "Bevy Sample STG\n".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/x8y12pxTheStrongGamer.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1., 1., 0.),
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
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "by Pocket7878".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/x8y12pxTheStrongGamer.ttf"),
                            font_size: 18.0,
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
        })
        .id();
    commands.insert_resource(TitleData { title_entity });
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
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
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/x8y12pxTheStrongGamer.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
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

fn cleanup_menu(mut commands: Commands, title_data: Res<TitleData>, menu_data: Res<MenuData>) {
    commands.entity(title_data.title_entity).despawn_recursive();
    commands.entity(menu_data.button_entity).despawn_recursive();
}
