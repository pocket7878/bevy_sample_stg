use bevy::prelude::*;

pub mod destroy_enemy;
pub mod enemy;
mod game_frame;
pub mod life_count;
pub mod play_area;
pub mod player;
pub mod player_shot;
pub mod player_stock;
pub mod scoreboard;
mod system_label;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(game_frame::GameFramePlugin)
            .add_plugin(play_area::PlayAreaPlugin)
            .add_plugin(scoreboard::ScoreBoardPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(player_shot::PlayerShotPlugin)
            .add_plugin(enemy::EnemyPlugin)
            .add_plugin(destroy_enemy::DestroyEnemyPlugin)
            .add_plugin(player_stock::PlayerStockPlugin);
    }
}
