mod app;
use app::App;
use eframe::egui;

pub fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Local Scope Example",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
