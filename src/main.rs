mod app;

use eframe::egui;

fn main() {
    let mut options = eframe::NativeOptions::default();
    let viewport = &mut options.viewport;
    viewport.inner_size = Some(egui::Vec2::new(900.0, 600.0));
    viewport.resizable = Some(false);
    let _ = eframe::run_native(
        "Missingno",
        options,
        Box::new(|_cc| Ok(Box::new(app::Missingno::new()))),
    );
}
