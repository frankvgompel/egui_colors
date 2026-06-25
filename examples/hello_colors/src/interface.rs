use crate::app::App;
use eframe::egui::{self, Ui};
use egui_colors::tokens::ThemeColor;

pub fn draw_interface(app: &mut App, ui: &mut Ui) {
    let names = vec!["Yellow", "YellowGreen", "Muted Purple"];
    let themes = vec![
        [ThemeColor::Custom([232, 210, 7]); 12],
        [ThemeColor::Custom([178, 194, 31]); 12],
        [ThemeColor::Custom([95, 78, 163]); 12],
    ];
    let custom = Some((names, themes));
    egui::Panel::top("t_panel").show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            app.colorix.light_dark_toggle_button(ui, 14.);
            ui.separator();
            ui.toggle_value(&mut app.util_bools[0], "Background Gradient");
            ui.separator();
            app.colorix.themes_dropdown(ui, custom, false);
        });
    });
    egui::Panel::left("left panel").show(ui, |ui| {
        if app.util_bools[0] {
            app.colorix.draw_background(ui.ctx(), false);
        }
        ui.add_space(20.);
        app.colorix.custom_picker(ui);
        ui.add_space(20.);
        app.colorix.ui_combo_12(ui, true);
    });
    app.demo.ui(ui);
    egui::CentralPanel::default().show(ui, |ui| {
        if app.util_bools[0] {
            app.colorix.draw_background(ui.ctx(), false);
        }
    });
}
