use crate::app::App;
use eframe::egui;
use egui_colors::tokens::ColorPreset;

pub fn draw_interface(app: &mut App, ctx: &egui::Context) {
    let names = vec!["Yellow", "YellowGreen", "Muted Purple"];
    let themes = vec![
        [ColorPreset::Custom([232, 210, 7]); 12],
        [ColorPreset::Custom([178, 194, 31]); 12],
        [ColorPreset::Custom([95, 78, 163]); 12],
    ];
    let custom = Some((names, themes));
    egui::TopBottomPanel::top("t_panel").show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| {
            app.colorix.light_dark_toggle_button(ui);
            ui.separator();
            ui.toggle_value(&mut app.util_bools[0], "Background Gradient");
            ui.separator();
            app.colorix.themes_dropdown(ui, custom, false);
        });
    });
    egui::SidePanel::left("left panel").show(ctx, |ui| {
        if app.util_bools[0] {
            app.colorix.draw_background(ctx, false);
        }
        ui.add_space(20.);
        app.colorix.custom_picker(ui);
        ui.add_space(20.);
        app.colorix.ui_combo_12(ui);
    });
    app.demo.ui(ctx);
    egui::CentralPanel::default().show(ctx, |_ui| {
        if app.util_bools[0] {
            app.colorix.draw_background(ctx, false);
        }
    });
}
