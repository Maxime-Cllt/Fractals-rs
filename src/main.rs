use eframe::egui;
use fractals_rs::constant::{HEIGHT, WIDTH};
use fractals_rs::structs::fractal_app::FractalApp;

#[cfg(test)]
mod benches;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WIDTH, HEIGHT]),
        ..Default::default()
    };

    eframe::run_native(
        "Fractal-rs",
        options,
        Box::new(|_cc| Ok(Box::<FractalApp>::default())),
    )
}
