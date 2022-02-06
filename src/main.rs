use bevy::prelude::*;
use minigame::MiniGamePlugin;

fn main() {
    App::new()
        .add_plugin(MiniGamePlugin)
        .run();
}
