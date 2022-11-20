// causes windows exe to run in the background
#![windows_subsystem = "windows"]

use std::{thread, time};

mod capture;
mod config;
mod log;

fn main() {
    let config = config::Config::init();

    loop {
        capture::capture_screens(&config);
        log::info("Screen capture done. Waiting for next interval.");
        thread::sleep(time::Duration::from_secs(config.interval_seconds));
    }
}
