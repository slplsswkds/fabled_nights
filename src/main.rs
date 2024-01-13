mod game_states;
mod settings;
use std::borrow::BorrowMut;

use bevy::prelude::*;
use settings::{load_settings_resource, Settings, AllSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_settings_resource)
        .run();
}