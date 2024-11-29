use std::collections::HashMap;

use egui::*;
use rfd::FileDialog;

use crate::app::{BottomMessage, Entry};

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

            if ui.button("Open Database").clicked() {
                let dir = FileDialog::new().set_directory("/").pick_folder();
                if let Some(path) = dir {
                    match sled::open(path) {
                        Ok(db) => {
                            self.items.borrow_mut().clear();
                            for i in db.iter() {
                                match i {
                                    Ok((key, value)) => {
                                        self.items.borrow_mut().insert(
                                            String::from_utf8_lossy(&key).to_string(),
                                            String::from_utf8_lossy(&value).to_string(),
                                        );
                                    }
                                    Err(err) => {
                                        *self.bottom_message.borrow_mut() =
                                            Some(BottomMessage::Error(format!(
                                                "Failed to read database: {}",
                                                err
                                            )));
                                    }
                                }
                            }
                        }

                        Err(err) => {
                            *self.bottom_message.borrow_mut() = Some(BottomMessage::Error(
                                format!("Failed to open database: {}", err),
                            ));
                        }
                    }
                }
            }
        });
    }
}
