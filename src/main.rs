mod game_states;
mod settings;
use bevy::prelude::*;
use settings::{load_settings_resource, Settings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_settings_resource)
        .add_systems(Update, fake)
        .run();
}

// треба розібратись як запускать це гавно з one-shot system (розібрався)
fn fake(world: &mut World) {
    settings::AllSettings::default().apply(world);
}
