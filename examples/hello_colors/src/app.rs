//#![allow(dead_code)]
use crate::interface;
use eframe::egui::{self, Ui};
use egui_colors::{utils, Colorix};
use egui_demo_lib::DemoWindows;
use std::sync::Arc;

#[derive(Default)]
pub struct App {
    pub colorix: Colorix,
    pub util_bools: [bool; 12],
    pub demo: DemoWindows,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        interface::draw_interface(self, ui);
    }
}

impl App {
    fn new(ctx: &egui::Context) -> Self {
        ctx.set_theme(egui::Theme::Light);
        let colorix = Colorix::global(ctx, utils::EGUI_THEME);
        Self {
            colorix,
            ..Default::default()
        }
    }
}

pub fn init() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "inter_medium".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../data/Inter-Medium.otf"
        ))),
    ); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "inter_medium".to_owned());

    // Put my font as last fallback for monospace:
    // fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
    //     .push("inter_medium".to_owned());
    eframe::run_native(
        "Egui Colors Demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            cc.egui_ctx.global_style_mut(|style| {
                style.spacing.item_spacing = egui::vec2(5.0, 8.0);
                style.spacing.window_margin = egui::Margin::same(20);
            });
            Ok(Box::new(App::new(&cc.egui_ctx.clone())))
        }),
    )
}
