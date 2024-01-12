use settings::Settings;

mod game_states;
mod settings;

use bevy::prelude::*;

fn main() {
    let _settings = Settings::get();

    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
