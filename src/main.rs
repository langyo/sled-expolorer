#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod pages;

use anyhow::anyhow;
use std::sync::Arc;

fn main() -> eframe::Result {
    // TODO: Read command line arguments to open a database folder

    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600., 400.])
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

pub fn replace_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "汉仪文黑-85W".to_owned(),
        egui::FontData::from_static(include_bytes!("../res/font.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .and_modify(|families| {
            families.insert(0, "汉仪文黑-85W".to_owned());
        });
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .and_modify(|families| {
            families.insert(0, "汉仪文黑-85W".to_owned());
        });

    ctx.set_fonts(fonts);
}
