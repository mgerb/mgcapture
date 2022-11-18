use std::fs::OpenOptions;
use std::io::prelude::*;

static LOG_FILE_NAME: &str = "mgcapture.log";

pub fn warn(msg: &str) {
    info(format!("WARN - {}", msg).as_str());
}

pub fn error(msg: &str) {
    info(format!("ERROR - {}", msg).as_str());
}

pub fn info(msg: &str) {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let message = format!("{}: {}", now, msg);

    println!("{}", message);

    // create file if not exists
    if !std::path::Path::new(LOG_FILE_NAME).exists() {
        std::fs::File::create(LOG_FILE_NAME).expect("Failed to create log file");
    }

    // read file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(LOG_FILE_NAME)
        .expect("Failed to open log file");

    writeln!(file, "{}", message).expect("Failed to write to log file");
}
