use bevy::prelude::*;
use bevy_sample_stg::GamePlugin;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .run();
}
