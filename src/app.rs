use std::{cell::RefCell, rc::Rc};

use crate::replace_fonts;

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Portal,
    Items(String),
    About,
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
        }
    }
}

impl eframe::App for Entry {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                    ui.vertical_centered_justified(|ui| {
                        let mut tag = tag.clone();
                        if ui
                            .selectable_value(&mut tag, Tag::Portal, "Portal")
                            .clicked()
                        {
                            self.tag.replace(tag.clone());
                        }
                        if ui.selectable_value(&mut tag, Tag::About, "About").clicked() {
                            self.tag.replace(tag.clone());
                        }

                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);
                        if match &tag {
                            Tag::Portal | Tag::About => false,
                            _ => true,
                        } {
                            let mut val = true;
                            ui.selectable_value(
                                &mut val,
                                true,
                                match &tag {
                                    Tag::Items(tree) => format!("#{tree}"),
                                    _ => panic!("Unknown tag: {:?}", self.tag),
                                },
                            );
                        }
                    });
                    egui::TopBottomPanel::bottom("drawer_tools")
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            let _ = ui.button("Refresh");
                        })
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| match &tag {
            Tag::Portal => {
                ui.label("TODO");
            }
            Tag::Items(_tree) => {
                ui.label("TODO");
            }
            Tag::About => {
                self.about_page_frame(ui);
            }
        });

        ctx.request_repaint();
    }
}
