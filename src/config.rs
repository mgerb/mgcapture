use std::{fs, path, string};

use serde::{Deserialize, Serialize};

static CONFIG_FILE_NAME: &str = "config.json";

fn default_output_directory() -> String {
    "screenshots".to_string()
}

fn default_interval_seconds() -> u64 {
    600
}

fn default_file_format() -> String {
    "%Y-%m-%d_%H-%M-%S".to_string()
}

fn default_folder_format() -> String {
    "%Y-%m-%d".to_string()
}

fn default_image_quality() -> u8 {
    80
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_output_directory")]
    pub output_directory: string::String,

    #[serde(default = "default_interval_seconds")]
    pub interval_seconds: u64,

    #[serde(default = "default_folder_format")]
    pub folder_format: string::String,

    #[serde(default = "default_file_format")]
    pub file_format: string::String,

    #[serde(default = "default_image_quality")]
    pub image_quality: u8,
}

impl Config {
    /// - create config file if it doesn't exist
    /// - read and parse config file
    /// - use default config if config file is invalid OR if any fields are missing use default
    /// values for those
    /// - write new config file
    /// - return config
    pub fn init() -> Self {
        // create file if not exists
        let config_file_path = path::Path::new(CONFIG_FILE_NAME);
        if !config_file_path.exists() {
            fs::write(config_file_path, "").expect("Failed to create config file.");
        }

        // read file config.json
        let config_file = fs::read_to_string(CONFIG_FILE_NAME).unwrap_or("".to_string());
        let config_file = serde_json::from_str(config_file.as_str());

        let mut config = Config {
            output_directory: default_output_directory(),
            interval_seconds: default_interval_seconds(),
            folder_format: default_folder_format(),
            file_format: default_file_format(),
            image_quality: default_image_quality(),
        };

        if let Ok(config_file) = config_file {
            config = config_file;
        }

        let config_string =
            serde_json::to_string_pretty(&config).expect("Error converting config to json");
        fs::write(config_file_path, config_string).expect("Failed to create config file.");

        return config;
    }
}
