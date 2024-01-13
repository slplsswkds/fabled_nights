use crate::settings::Settings;
use bevy::{ecs::system::RunSystemOnce, prelude::*, window::WindowMode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WindowSettings {
    width: usize,
    height: usize,
    window_mode: WindowMode,
    vsync: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 640,
            height: 480,
            window_mode: WindowMode::Windowed,
            vsync: false,
        }
    }
}

impl Settings for WindowSettings {
    fn apply(&self, world: &mut World) {
        world.run_system_once(apply_window_settings);
    }
}

fn apply_window_settings(mut focused_windows: Query<(Entity, &mut Window)>) {
    for (_, mut window) in focused_windows.iter_mut() {
        window.mode = bevy::window::WindowMode::Fullscreen;
    }
}
