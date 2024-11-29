use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::theme::replace_fonts;

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Portal,
    Items(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BottomMessage {
    Info(String),
    Error(String),
}

pub struct Entry {
    pub tag: Rc<RefCell<Tag>>,
    pub is_loading: Rc<RefCell<bool>>,
    pub bottom_message: Rc<RefCell<Option<BottomMessage>>>,

    pub items: Rc<RefCell<HashMap<String, String>>>,
}

impl Entry {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        replace_fonts(&cc.egui_ctx);

        Self::default()
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            tag: Rc::new(RefCell::new(Tag::Portal)),
            is_loading: Rc::new(RefCell::new(false)),
            bottom_message: Rc::new(RefCell::new(None)),

            items: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl eframe::App for Entry {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.heading("Sled Database Explorer");
                ui.add_space(4.0);
            });
        });

        let tag = self.tag.borrow().clone();

        egui::SidePanel::left("drawer")
            .resizable(false)
            .default_width(128.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(4.0);
                    ui.vertical_centered_justified(|ui| {
                        let mut tag = tag.clone();
                        if ui
                            .selectable_value(&mut tag, Tag::Portal, "Portal")
                            .clicked()
                        {
                            self.tag.replace(tag.clone());
                        }

                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);
                        // if match &tag {
                        //     Tag::Portal => false,
                        //     _ => true,
                        // } {
                        //     let mut val = true;
                        //     ui.selectable_value(
                        //         &mut val,
                        //         true,
                        //         match &tag {
                        //             Tag::Items(tree) => format!("#{tree}"),
                        //             _ => panic!("Unknown tag: {:?}", self.tag),
                        //         },
                        //     );
                        // }

                        // FIXME: Don't show this tag if the folder is not opened
                        if ui
                            .selectable_value(
                                &mut tag,
                                Tag::Items("<Default>".to_string()),
                                "<Default>".to_string(),
                            )
                            .clicked()
                        {
                            self.tag.replace(Tag::Items("<Default>".to_string()));
                        }
                    });
                    egui::TopBottomPanel::bottom("drawer_tools")
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            ui.add_space(4.0);
                            ui.vertical_centered(|ui| {
                                if let Some(bottom_message) = self.bottom_message.borrow().clone() {
                                    match bottom_message {
                                        BottomMessage::Info(message) => {
                                            ui.label(message);
                                        }
                                        BottomMessage::Error(message) => {
                                            ui.colored_label(egui::Color32::RED, message);
                                        }
                                    }
                                    ui.add_space(4.0);
                                }

                                if ui.button("Save").clicked() {
                                    self.bottom_message.replace(Some(BottomMessage::Info(
                                        "Save clicked".to_string(),
                                    )));
                                }

                                ui.add_space(4.0);

                                if ui.button("Restore").clicked() {
                                    self.bottom_message.replace(Some(BottomMessage::Info(
                                        "Restore clicked".to_string(),
                                    )));
                                }
                            });
                            ui.add_space(4.0);
                        })
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| match &tag {
            Tag::Portal => {
                self.portal_page_frame(ui);
            }
            Tag::Items(tree_key) => {
                self.items_page_frame(ui, tree_key.clone());
            }
        });

        ctx.request_repaint();
    }
}
