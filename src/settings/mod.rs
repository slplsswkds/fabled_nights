mod mouse;
mod window;

use bevy::prelude::*;
use mouse::MouseSettings;
use serde::{Deserialize, Serialize};
use window::WindowSettings;

const SETTINGS_PATH: &str = "./settings.yaml";

/// Stores all settings that can be configured from userspace
#[derive(Resource, Serialize, Deserialize, Default, Clone)]
pub struct AllSettings {
    window: WindowSettings,
    mouse: MouseSettings,
}

impl AllSettings {
    /// Serializes AllSettings to YAML and writes them to the settings file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(SETTINGS_PATH)?;
        serde_yaml::to_writer(settings_file, &self)?;
        Ok(())
    }

    /// Returns the AllSettings read from the settings file.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let settings_file = std::fs::File::open(SETTINGS_PATH)?;
        let settings = serde_yaml::from_reader(settings_file)?;
        Ok(settings)
    }

    /// Get AllSettings from file or use defaults, saving them
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

/// Adds the Settings resource
pub fn load_settings_resource(mut commands: Commands) {
    let settings = AllSettings::get();
    commands.insert_resource(settings);
}

pub trait Settings {
    /// Overwrites the AllSettings resource with new values
    fn save_to_resource(&self, world: &mut World);

    /// Applies the new settings (like changing Window proporties, etc...)
    fn apply(&self, world: &mut World);
}

impl Settings for AllSettings {
    fn save_to_resource(&self, world: &mut World) {
        self.window.save_to_resource(world);
        self.mouse.save_to_resource(world);
    }

    fn apply(&self, world: &mut World) {
        self.window.apply(world);
        self.mouse.apply(world);
    }
}
