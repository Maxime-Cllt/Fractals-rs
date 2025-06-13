use crate::constant::{HEIGHT, WIDTH};
use crate::fractal_app::FractalApp;
use eframe::egui;

mod constant;
mod fractal_app;
mod frame;
mod fractals;
mod color_scheme;
mod point;

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
