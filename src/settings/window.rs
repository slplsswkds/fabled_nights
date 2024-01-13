use crate::settings::{AllSettings, Settings};
use bevy::{ecs::system::RunSystemOnce, prelude::*, window::WindowMode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WindowSettings {
    width: usize,
    height: usize,
    mode: WindowMode,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 640,
            height: 480,
            mode: WindowMode::Windowed,
        }
    }
}

impl Settings for WindowSettings {
    fn apply(&self, world: &mut World) {
        world.run_system_once(apply_window_settings);
    }

    fn save_to_resource(&self, world: &mut World) {
        let mut settings_resource = world
            .get_resource_mut::<AllSettings>()
            .expect("Resource AllSettings is not aailable");

        settings_resource.window.width = self.width;
        settings_resource.window.height = self.height;
        settings_resource.window.mode = self.mode;
    }
}

///
/// Overwrites the WindowSettings with new values and applies the new settings for the Window
fn apply_window_settings(
    mut windows: Query<&mut Window>, 
    settings: Res<AllSettings>
) {
    let mut window = windows
        .get_single_mut()
        .expect("No one window found or them more than one");

    window.mode = settings.window.mode;
    window.resolution = bevy::window::WindowResolution::new(
        settings.window.width as f32,
        settings.window.height as f32,
    );
}