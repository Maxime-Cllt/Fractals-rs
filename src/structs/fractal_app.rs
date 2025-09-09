use crate::enums::fractal_type::FractalType;
use crate::enums::precision_mode::PrecisionMode;
use crate::structs::color_scheme::ColorScheme;
use crate::structs::point::Point;

/// The main application state for the fractal viewer.
pub struct FractalApp {
    pub fractal_type: FractalType,
    pub max_iterations: u16,
    pub center: Point,
    pub zoom: f64,
    pub julia_c: Point,
    pub needs_update: bool,
    pub texture: Option<egui::TextureHandle>,
    pub image_size: (u32, u32),
    pub is_dragging: bool,
    pub show_settings: bool,
    pub precision_mode: PrecisionMode,
    pub color_scheme: ColorScheme,
}
