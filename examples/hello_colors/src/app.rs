//#![allow(dead_code)]
use crate::interface;
use eframe::egui;
use egui_colors::{utils, Colorix};
use egui_demo_lib::DemoWindows;

#[derive(Default)]
pub struct App {
    pub colorix: Colorix,
    pub util_bools: [bool; 12],
    pub demo: DemoWindows,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        interface::draw_interface(self, ctx);
    }
}

impl App {
    fn new(ctx: &mut egui::Context) -> Self {
        let colorix = Colorix::init(ctx, utils::EGUI_THEME);
        Self {
            colorix,
            ..Default::default()
        }
    }
}

pub fn init() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "inter_medium".to_owned(),
        egui::FontData::from_static(include_bytes!("../data/Inter-Medium.otf")),
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
            cc.egui_ctx.style_mut(|style| {
                style.spacing.item_spacing = egui::vec2(5.0, 8.0);
                style.spacing.window_margin = egui::Margin::same(20.0);
            });
            Ok(Box::new(App::new(&mut cc.egui_ctx.clone())))
        }),
    )
}
