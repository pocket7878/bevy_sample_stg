use crate::app_state::AppState;
use crate::play_area::PlayAreaDescriptor;
use bevy::prelude::*;

const PLAYER_SIZE: f32 = 30.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(move_player_by_keyboard_system),
            );
    }
}

#[derive(Component)]
pub struct Player;

fn setup(
    play_area: Res<PlayAreaDescriptor>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let rocket_asset_handle: Handle<Image> = asset_server.load("images/rocket.png");

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    play_area.origin.x - PLAYER_SIZE / 2.0,
                    play_area.origin.y
                        - (play_area.height / 2. - PLAYER_SIZE * 3.0 - PLAYER_SIZE / 2.0),
                    0.0,
                ),
                scale: Vec3::new(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            texture: rocket_asset_handle.into(),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player_by_keyboard_system(
    play_area: Res<PlayAreaDescriptor>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (_, mut transform) = query.single_mut();

    // 斜め移動も考慮して比率計算
    let move_ratio;
    if (keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D))
        && (keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::S))
    {
        move_ratio = 0.71;
    } else {
        move_ratio = 1.0;
    }

    let move_dist = 1.0;
    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= move_dist * move_ratio;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += move_dist * move_ratio;
    }
    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += move_dist * move_ratio;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= move_dist * move_ratio;
    }

    transform.translation.x = transform
        .translation
        .x
        .min(play_area.max_x() - transform.scale.x)
        .max(play_area.min_x() + transform.scale.y);

    transform.translation.y = transform
        .translation
        .y
        .min(play_area.max_y() - transform.scale.y / 2.0)
        .max(play_area.min_y() + transform.scale.y / 2.0);
}
