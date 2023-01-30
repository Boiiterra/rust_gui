#![windows_subsystem = "windows"]

mod theme;

use theme::CustomTheme;

use eframe::{
    egui::{FontData, epaint::FontFamily, FontDefinitions, style::Margin, RichText},
    run_native, App, NativeOptions, Theme, egui, epaint::FontId, IconData
};

struct RustGui {
    theme: CustomTheme,
}

impl App for RustGui {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui: &mut egui::Ui| {

            ctx.set_pixels_per_point(1.5f32);

            let central_panel_frame = egui::containers::Frame {
                inner_margin: Margin::symmetric(10f32, 10f32),
                outer_margin: Margin::symmetric(10f32, 10f32),
                rounding: egui::Rounding {
                    nw: 10.0,
                    ne: 10.0,
                    sw: 10.0,
                    se: 10.0,
                },
                fill: self.theme.bg_color(),
                ..Default::default()
            };

            egui::CentralPanel::default()
                .frame(central_panel_frame)
                .show(ctx, |ui| {    
                    //Top heading
                    ui.vertical_centered_justified(|ui| {
                        ui.heading(RichText::new("Heading text.").color(self.theme.fg_color()));
                        ui.separator();

                        ui.label(RichText::new("Some text.").color(self.theme.fg_color()));
                    });
                })
                .response
                .context_menu(|ui| {
    
                    ui.menu_button("Theme", |ui| {
                        ui.selectable_value(&mut self.theme, CustomTheme::Dark, "Dark");
                        ui.selectable_value(&mut self.theme, CustomTheme::Light, "Light");
                    });

                    ui.menu_button("From", |ui| {
                        ui.vertical(|ui| {
                            ui.hyperlink_to("|>  TerraBoii  <|", "https://github.com/TerraBoii");
                        });
                    });

                    if ui.button("Do not click me!").clicked() {
                        ui.output().open_url = Some(egui::output::OpenUrl {
                            new_tab: true,
                            url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
                        });
                    }
                });
            ctx.request_repaint();
        });
    }
}

impl Default for RustGui {
    fn default() -> Self {
        Self {
            theme: CustomTheme::Dark,
        }
    }
}

impl RustGui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        //Init default fonts to load custom along with default
        let mut fonts = FontDefinitions::default();

        //Load custom font
        fonts.font_data.insert(
            "Undertale".to_owned(),
            FontData::from_static(include_bytes!("res/Undertale-Battle-Font.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Undertale".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("Undertale".to_owned());

        //Use custom fonts
        cc.egui_ctx.set_fonts(fonts);

        //Initial `pixels_per_point`
        cc.egui_ctx.set_pixels_per_point(1.5f32);

        use egui::FontFamily::Proportional;
        use egui::TextStyle::*;
        let mut style = (*cc.egui_ctx.style()).clone();

        style.text_styles = [
            (Heading, FontId::new(25.0, Proportional)),
            (Body, FontId::new(14.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();

        cc.egui_ctx.set_style(style);

        Self::default()
    }
}

fn main() {
    let native_options = NativeOptions {
        icon_data: Some(IconData {
            rgba: include_bytes!("res/icon.rgba").to_vec(),
            width: 32,
            height: 32,
        }),
        initial_window_pos: Some(egui::pos2(660f32, 315f32)),
        initial_window_size: Some(egui::vec2(400f32, 300f32)),
        resizable: false,
        transparent: true,
        default_theme: Theme::Dark,
        ..Default::default()
    };

    run_native(
        "Window title",
        native_options,
        Box::new(|cc| Box::new(RustGui::new(cc))),
    );
}
