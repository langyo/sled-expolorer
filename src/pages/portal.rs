use egui::*;

use crate::app::Entry;

#[derive(Clone, Debug, PartialEq)]
pub struct PortalPage {}

impl Default for PortalPage {
    fn default() -> Self {
        Self {}
    }
}

impl Entry {
    pub fn portal_page_frame(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.set_min_width(ui.available_width());

            ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
            ui.label("原神，启动！");
        });
    }
}
