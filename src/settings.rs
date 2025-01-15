use fmc::prelude::*;

use std::{
    hash::{DefaultHasher, Hasher},
    io::{BufRead, BufReader},
};

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings::load());
    }
}

#[derive(Resource)]
pub struct Settings {
    /// Name of the world that should be loaded
    pub database_path: String,
    /// Seed used for terrain generation
    pub seed: u64,
    /// Should pvp be enabled
    pub pvp: bool,
    /// The max render distance the server will provide for.
    pub render_distance: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database_path: "world.sqlite".to_owned(),
            seed: 1,
            pvp: false,
            render_distance: 16,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let mut server_settings = Settings::default();

        let path = "./server_settings.txt";
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => {
                Self::write_default();
                return server_settings;
            }
        };
        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.unwrap();

            // comments
            if line.starts_with("#") {
                continue;
            }

            let (name, value) = line.split_once("=").unwrap_or_else(|| {
                panic!(
                    "Error reading server_settings.txt at line {}.
                       All settings must be of the format 'name = setting', it cannot be '{}'",
                    line_num, line
                );
            });
            let name = name.trim();
            let value = value.trim();

            match name {
                "world-name" => {
                    server_settings.database_path = "./".to_owned() + value + ".sqlite";
                }
                "seed" => {
                    let mut hasher = DefaultHasher::new();
                    hasher.write(value.as_bytes());
                    server_settings.seed = hasher.finish();
                }
                "pvp" => {
                    let value = value.parse::<bool>().unwrap_or_else(|_| {
                        panic!(
                            "Server property 'pvp' must be one of 'true/false', cannot be: {}",
                            value
                        )
                    });
                    server_settings.pvp = value;
                }
                _ => {
                    panic!("Invalid setting '{name}' in settings file at line {line}",);
                }
            }
        }

        return server_settings;
    }

    // Writes a default config to the server directory.
    #[rustfmt::skip]
    fn write_default() {
        let settings = Self::default();
        let contents = String::new()
            + "#world-name = " + &settings.database_path + "\n"
            + "#pvp = " + &settings.pvp.to_string();

        std::fs::write("./server_settings.txt", contents).unwrap();
    }
}
