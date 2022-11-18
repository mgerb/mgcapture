use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub output_directory: std::string::String,
    pub interval_seconds: u64,
    pub folder_format: std::string::String,
    pub file_format: std::string::String,
}

impl Config {
    pub fn init() -> Self {
        // read file config.json
        let config_file = std::fs::read_to_string("config.json");

        // config doesn't already exist
        if config_file.is_err() {
            let config = Config {
                output_directory: std::path::Path::new("screenshots")
                    .to_str()
                    .expect("Error creating screenshots path")
                    .to_string(),
                interval_seconds: 600,
                folder_format: "%Y-%m-%d".to_string(),
                file_format: "%Y-%m-%d_%H-%M-%S".to_string(),
            };

            // convert config to pretty json string
            let config_json =
                serde_json::to_string_pretty(&config).expect("Error converting config to json");

            // write config to file if file doesn't exist
            if !std::path::Path::new("config.json").exists() {
                std::fs::write("config.json", config_json).expect("Failed to write config to file");
            }

            return config;
        } else {
            // deserialize config from json
            return serde_json::from_str(&config_file.unwrap())
                .expect("Failed to deserialize config - invalid config.json file");
        }
    }
}
