use eframe::{egui, NativeOptions};
use egui::IconData;
use fractals_rs::constant::{HEIGHT, WIDTH};
use fractals_rs::structs::fractal_app::FractalApp;

#[cfg(test)]
mod benches;

fn main() -> Result<(), eframe::Error> {
    let icon_data: Result<IconData, eframe::Error> = load_icon();

    let options: NativeOptions = if let Ok(icon_data) = icon_data {
        NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([WIDTH, HEIGHT])
                .with_icon(icon_data),
            ..Default::default()
        }
    } else {
        NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([WIDTH, HEIGHT]),
            ..Default::default()
        }
    };

    eframe::run_native(
        "Fractal-rs",
        options,
        Box::new(|_cc| Ok(Box::<FractalApp>::default())),
    )
}

fn load_icon() -> Result<IconData, eframe::Error> {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/fractale.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Ok(IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    })
}
