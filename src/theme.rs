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
