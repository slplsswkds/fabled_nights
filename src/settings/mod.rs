use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = "settings.yaml";

#[derive(Serialize, Deserialize, Default)]
enum WindowMode {
    #[default]
    Fullscreen,
    Windowed,
    WindowedNoTitle,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct Settings {
    // Window
    resolution_x: usize,
    resolution_y: usize,
    window_mode: WindowMode,
    vsync: bool,
    // Mouse
    mouse_sensivity_x: f32,
    mouse_sensivity_y: f32,
}

impl Settings {
    /// Serializes Settings in YAML and writes them to the settings file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(SETTINGS_PATH)?;
        serde_yaml::to_writer(settings_file, &self)?;
        Ok(())
    }

    /// Returns the settings read from the settings file.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let settings_file = std::fs::File::open(SETTINGS_PATH)?;
        let settings = serde_yaml::from_reader(settings_file)?;
        Ok(settings)
    }

    /// Get Settings from file or use defaults, saving them
    pub fn get() -> Self {
        match Self::load() {
            Ok(settings) => settings,
            Err(_) => {
                let defaults = Self::default();
                if defaults.save().is_err() {
                    eprintln!("Failed to save settings file ")
                }
                defaults
            }
        }
    }
}
