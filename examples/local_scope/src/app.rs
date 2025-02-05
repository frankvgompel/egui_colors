use eframe::egui;
use egui_colors::{utils, Colorix};

#[derive(Default)]
pub struct App {
    pub colorix: Colorix,
    pub colorix2: Colorix,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.colorix.set_animator(ctx);
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            self.colorix.light_dark_toggle_button(ui, 14.);
            ui.add_space(10.);
            self.colorix.themes_dropdown(ui, None, false);
            ui.add_space(10.);
            self.colorix.ui_combo_12(ui, false);
        });
        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::same(8),
                fill: self.colorix2.animator.animated_tokens.subtle_background(),
                ..Default::default()
            })
            .show(ctx, |ui| {
                if ui.button("init local colorix").clicked() {
                    self.colorix2 = Colorix::local(ui, utils::COOL).animated();
                }
                self.colorix2.update_locally(ui);
                ui.add_space(10.);
                self.colorix2.themes_dropdown(ui, None, false);
                ui.add_space(10.);
                self.colorix2.ui_combo_12(ui, false);
                ui.add_space(10.);
                if ui.button("Click for dark").clicked() {
                    self.colorix2.set_dark(ui);
                };
                if ui.button("Click for light").clicked() {
                    self.colorix2.set_light(ui)
                };
            });
    }
}
impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Light);
        let colorix = Colorix::global(&cc.egui_ctx, utils::EGUI_THEME).animated();
        Self {
            colorix,
            ..Default::default()
        }
    }
}
