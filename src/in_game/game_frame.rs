use crate::{app_state::AppState, FPS};
use bevy::prelude::*;

use super::system_label::GameSystemLabel;

pub struct GameFramePlugin;

impl Plugin for GameFramePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_game_frame_system)
                    .label(GameSystemLabel::GameFrameUpdate),
            );
    }
}

struct GameFrame(i128);

impl Default for GameFrame {
    fn default() -> Self {
        GameFrame(0)
    }
}

struct GameFrameTimer(Timer);

impl Default for GameFrameTimer {
    fn default() -> Self {
        GameFrameTimer(Timer::from_seconds(1.0 / FPS, true))
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(GameFrame::default());
}

fn update_game_frame_system(
    time: Res<Time>,
    mut timer: ResMut<GameFrameTimer>,
    mut game_frame: ResMut<GameFrame>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        game_frame.0 += 1;
    }
}
