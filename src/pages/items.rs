use egui::*;
use egui_extras::{Column, TableBuilder};

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

            ScrollArea::horizontal().show(ui, |ui| {
                ui.set_min_width(ui.available_width());

                let mut table = TableBuilder::new(ui)
                    .striped(false)
                    .resizable(false)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .min_scrolled_height(0.0)
                    .auto_shrink([false; 2]);

                let state = self.items.borrow().clone();
                for _ in 0..3 {
                    table = table.column(Column::auto());
                }
                table = table.column(Column::remainder());

                table
                    .header(24.0, |mut row| {
                        row.col(|ui| {
                            if ui.button("Filter").clicked() {
                                // TODO: Open filter dialog
                            };
                        });
                        row.col(|ui| {
                            ui.strong("Key");
                        });
                        row.col(|ui| {
                            ui.strong("Value");
                        });
                    })
                    .body(|mut table_body| {
                        for (key, value) in state.iter() {
                            let mut is_pass = true;
                            // TODO: Add filter
                            if !is_pass {
                                continue;
                            }

                            table_body.row(24.0, |mut row| {
                                row.col(|ui| {
                                    if ui.button("Delete").clicked() {
                                        // TODO: Delete item
                                    };
                                });
                                row.col(|ui| {
                                    ui.label(key);
                                });
                                row.col(|ui| {
                                    ui.label(value);
                                });
                            });
                        }

                        table_body.row(24.0, |mut ui| {
                            ui.col(|ui| {
                                if ui.button("Add").clicked() {
                                    // TODO: Add item
                                };
                            });
                            ui.col(|ui| {
                                if ui.button("Load more").clicked() {
                                    // TODO: Load more items
                                };
                            });
                        });
                    });
            });
        });
    }
}
