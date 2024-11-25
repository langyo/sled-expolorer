#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod pages;
pub mod theme;

use anyhow::anyhow;
use std::sync::Arc;

fn main() -> eframe::Result {
    // TODO: Read command line arguments to open a database folder

    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800., 600.])
            .with_icon(Arc::new({
                let icon = include_bytes!("../res/logo.png");
                let icon = image::load_from_memory(icon).map_err(|err| {
                    eframe::Error::AppCreation(anyhow!("Failed to load icon: {}", err).into())
                })?;
                egui::IconData {
                    rgba: icon.to_rgba8().into_raw(),
                    width: icon.width(),
                    height: icon.height(),
                }
            })),
        centered: true,

        ..Default::default()
    };

    eframe::run_native(
        "Sled Database Explorer",
        options,
        Box::new(|cc| Ok(Box::new(app::Entry::new(cc)))),
    )
}
