use egui::*;

use crate::app::Entry;

#[derive(Clone, Debug, PartialEq)]
pub struct ItemsPage {}

impl Default for ItemsPage {
    fn default() -> Self {
        Self {}
    }
}

impl Entry {
    pub fn items_page_frame(&mut self, ui: &mut Ui, tree_key: String) {
        ui.vertical(|ui| {
            ui.set_min_width(ui.available_width());

            ui.label(format!("key: {}", tree_key));
            ui.label("原神，启动！");
        });
    }
}
