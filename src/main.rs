// causes windows exe to run in the background
#![windows_subsystem = "windows"]

use std::{fs, path, thread, time};

use screenshots::Screen;

mod config;
mod log;

fn main() {
    let config = config::Config::init();

    loop {
        capture_screens(&config);
        log::info("Screen capture done. Waiting for next interval.");
        thread::sleep(time::Duration::from_secs(config.interval_seconds));
    }
}

fn capture_screens(config: &config::Config) {
    if let Some(screens) = Screen::all() {
        // get todays date
        let folder_name = chrono::Local::now()
            .format(config.folder_format.as_str())
            .to_string();
        let file_name = chrono::Local::now()
            .format(config.file_format.as_str())
            .to_string();

        for (index, screen) in screens.iter().enumerate() {
            match screen.capture() {
                Some(image) => {
                    let buffer = image.buffer();

                    let path =
                        path::Path::new(config.output_directory.as_str()).join(folder_name.clone());
                    let file_path = path.join(format!("{}-{}.png", file_name, index));

                    log::info(file_path.as_os_str().to_str().unwrap());
                    match fs::create_dir_all(path.into_os_string()) {
                        Ok(_) => match fs::write(file_path.into_os_string(), &buffer) {
                            Ok(_) => {
                                log::info(
                                    format!("Saved screenshot for screen {}.", index).as_str(),
                                );
                            }
                            Err(e) => {
                                log::error(format!("Failed to save screenshot: {}", e).as_str());
                            }
                        },
                        Err(e) => {
                            log::error(format!("Failed to create directory: {}", e).as_str());
                        }
                    }
                }
                None => log::error(format!("Failed to capture screen: {}", index).as_str()),
            }
        }
    } else {
        log::error("No screens found.");
    }
}
