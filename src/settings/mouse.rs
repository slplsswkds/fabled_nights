use crate::settings::Settings;
use bevy::{ecs::system::RunSystemOnce, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MouseSettings {
    mouse_sensivity_x: f32,
    mouse_sensivity_y: f32,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            mouse_sensivity_x: 1.0,
            mouse_sensivity_y: 1.0,
        }
    }
}

impl Settings for MouseSettings {
    fn apply(&self, world: &mut World) {
        world.run_system_once(apply_mouse_settings);
    }
}

fn apply_mouse_settings() {}
