use super::player::Player;
use crate::app_state::AppState;
use crate::in_game::play_area::PlayAreaDescriptor;
use bevy::prelude::*;

const BULLET_SIZE: f32 = 15.0;

pub struct PlayerShotPlugin;

impl Plugin for PlayerShotPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(shot_player_bullet_by_keyboard_system)
                .with_system(repeat_player_shot_by_timer_system)
                .with_system(destroy_player_bullet_go_outside_system)
                .with_system(move_player_bullet_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup));
    }
}

/*
 * Component
 */
#[derive(Component)]
pub struct Bullet;

struct ShotPlayerBulletTimer(Timer);

impl ShotPlayerBulletTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(1.0 / 10.0, true))
    }
}

/*
 * System
 */
fn cleanup(mut commands: Commands, player_bullet_query: Query<Entity, With<Bullet>>) {
    commands.remove_resource::<ShotPlayerBulletTimer>();
    for bullet_entity in player_bullet_query.iter() {
        commands.entity(bullet_entity).despawn_recursive();
    }
}

fn shot_player_bullet_by_keyboard_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&Player, &Transform)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        start_repeat_player_bullet_shot_timer(&mut commands);
    } else if keyboard_input.just_released(KeyCode::Space) {
        stop_repeat_player_bullet_shot_timer(&mut commands);
        return;
    } else {
        return;
    }

    let (_, transform) = query.single();
    shot_player_bullet(commands, transform);
}

fn repeat_player_shot_by_timer_system(
    commands: Commands,
    time: Res<Time>,
    mut timer: Option<ResMut<ShotPlayerBulletTimer>>,
    player_query: Query<(&Player, &Transform)>,
) {
    if let Some(ref mut timer) = timer {
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        let (_, transform) = player_query.single();
        shot_player_bullet(commands, transform);
    }
}

fn move_player_bullet_system(mut query: Query<(&Bullet, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation.y += 10.0;
    }
}

fn destroy_player_bullet_go_outside_system(
    play_area: Res<PlayAreaDescriptor>,
    mut commands: Commands,
    player_bullet_query: Query<(Entity, &Bullet, &Transform)>,
) {
    for (player_bullet_entity, _, player_bullet_transform) in player_bullet_query.iter() {
        let player_bullet_bottom_y = player_bullet_transform.translation.y - BULLET_SIZE;
        if player_bullet_bottom_y > play_area.max_y() {
            commands.entity(player_bullet_entity).despawn();
        }
    }
}

/*
 * Utils
 */
fn start_repeat_player_bullet_shot_timer(commands: &mut Commands) {
    commands.insert_resource(ShotPlayerBulletTimer::new());
}

fn stop_repeat_player_bullet_shot_timer(commands: &mut Commands) {
    commands.remove_resource::<ShotPlayerBulletTimer>();
}

fn shot_player_bullet(mut commands: Commands, player_transform: &Transform) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    0.0,
                ),
                scale: Vec3::new(BULLET_SIZE, BULLET_SIZE, BULLET_SIZE),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bullet);
}
