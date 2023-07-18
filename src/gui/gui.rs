// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::env;

use dotenv::dotenv;
use log::{info, warn};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    dotenv().ok();

    simple_logger::init_with_env().unwrap();

    info!("Project Janus");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(janus::gui::SimulatorGUI::new(cc))),
    );

    Ok(())
}
