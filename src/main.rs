use eframe::{NativeOptions, egui};
use egui::IconData;
use fractals_rs::constant::{HEIGHT, WIDTH};
use fractals_rs::ui::fractal_app::FractalApp;

const APP_ICON: &[u8] = include_bytes!("../assets/fractale.png");

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
        let image = image::load_from_memory(APP_ICON)
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
