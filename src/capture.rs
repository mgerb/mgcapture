use image::io::Reader as ImageReader;
use std::{fs, io::Cursor, path};

use screenshots::Screen;

use crate::{config, log};

pub fn capture_screens(config: &config::Config) {
    let screens = Screen::all();

    if screens.is_none() {
        log::error("No screens found.");
        return;
    }

    let screens = screens.unwrap();

    // get todays date
    let folder_name = chrono::Local::now()
        .format(config.folder_format.as_str())
        .to_string();
    let file_name = chrono::Local::now()
        .format(config.file_format.as_str())
        .to_string();

    for (index, screen) in screens.iter().enumerate() {
        let image = match screen.capture() {
            Some(image) => image,
            None => {
                log::error(format!("Failed to capture screen: {}", index).as_str());
                continue;
            }
        };

        // The screenshots libray creates a PNG so we need to convert
        // it to a JPG. This isn't the most ideal, but it's fine
        // because speed isn't much of a concern here. Ideally we
        // would modify the screenshots lib to optionally return a JPG.
        let bytes = match convert_to_jpg(&config, image.buffer()) {
            Ok(bytes) => bytes,
            Err(e) => {
                log::error(format!("Failed to convert image to jpg: {}", e).as_str());
                continue;
            }
        };

        // get paths
        let dir_path = path::Path::new(config.output_directory.as_str()).join(folder_name.clone());
        let file_path = dir_path.join(format!("{}-{}.jpg", file_name, index));

        // create directory
        if let Err(e) = fs::create_dir_all(dir_path.into_os_string()) {
            log::error(format!("Failed to create directory: {}", e).as_str());
            continue;
        }

        // write the new image
        if let Err(e) = fs::write(file_path.clone().into_os_string(), &bytes) {
            log::error(format!("Failed to save screenshot: {}", e).as_str());
            continue;
        }

        log::info(
            format!(
                "Saved screenshot for screen {}: {}",
                index,
                file_path.to_str().unwrap_or("path error")
            )
            .as_str(),
        );
    }
}

fn convert_to_jpg(
    config: &config::Config,
    bytes: &Vec<u8>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = ImageReader::new(Cursor::new(bytes.as_slice()))
        .with_guessed_format()?
        .decode()?;

    let mut bytes: Vec<u8> = Vec::new();

    img.write_to(
        &mut Cursor::new(&mut bytes),
        image::ImageOutputFormat::Jpeg(config.image_quality),
    )?;

    return Ok(bytes);
}
