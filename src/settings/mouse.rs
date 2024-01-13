use crate::settings::{AllSettings, Settings};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MouseSettings {
    sensivity_x: f32,
    sensivity_y: f32,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            sensivity_x: 1.0,
            sensivity_y: 1.0,
        }
    }
}

impl Settings for MouseSettings {
    fn save_to_resource(&self, world: &mut World) {
        let mut settings_resource = world
            .get_resource_mut::<AllSettings>()
            .expect("Resource AllSettings is not aailable");

        settings_resource.mouse.sensivity_x = self.sensivity_x;
        settings_resource.mouse.sensivity_y = self.sensivity_y;
    }

    fn apply(&self, _world: &mut World) {}
}
