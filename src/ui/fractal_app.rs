use crate::enums::fractal_type::FractalType;
use crate::enums::precision_mode::PrecisionMode;
use crate::structs::color_scheme::ColorScheme;
use crate::structs::fractal_app::FractalApp;
use crate::structs::point::Point;
use egui::{Color32, Vec2};
use rayon::prelude::*;

impl Default for FractalApp {
    /// Creates a default instance of `FractalApp` with predefined settings.
    #[inline]
    fn default() -> Self {
        Self {
            fractal_type: FractalType::Mandelbrot,
            max_iterations: 300,
            center: Point::new(-0.5, 0.0),
            zoom: 1.0,
            julia_c: Point::new(-0.7269, 0.1889),
            needs_update: true,
            texture: None,
            image_size: (800, 600),
            is_dragging: false,
            show_settings: false,
            precision_mode: PrecisionMode::Fast,
            color_scheme: ColorScheme::default(),
        }
    }
}

impl FractalApp {
    /// Generates a fractal image based on the current settings.
    #[inline]
    #[must_use]
    pub fn generate_fractal_image(&self) -> egui::ColorImage {
        let width = self.image_size.0 as usize;
        let height = self.image_size.1 as usize;

        if width == 0 || height == 0 {
            return egui::ColorImage::new([1, 1], Color32::BLACK);
        }

        let (x_scale, y_scale, x_min, y_min) = self.compute_scale();

        let pixels: Vec<Color32> = (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width).into_par_iter().map(move |x| {
                    let cx = (x as f64).mul_add(x_scale, x_min);
                    let cy = (y as f64).mul_add(y_scale, y_min);

                    let iterations = self.fractal_type.iterations(
                        cx,
                        cy,
                        self.max_iterations,
                        &self.julia_c,
                        self.precision_mode,
                    );

                    self.color_scheme
                        .to_color32(iterations, self.max_iterations)
                })
            })
            .collect();

        egui::ColorImage::from_rgba_unmultiplied(
            [width, height],
            &pixels
                .into_iter()
                .flat_map(|c| [c.r(), c.g(), c.b(), c.a()])
                .collect::<Vec<u8>>(),
        )
    }

    /// Computes the scale factors and min/max coordinates for the fractal view.
    #[inline]
    fn compute_scale(&self) -> (f64, f64, f64, f64) {
        let width: u32 = self.image_size.0;
        let height: u32 = self.image_size.1;

        let aspect_ratio: f64 = f64::from(width) / f64::from(height);
        let zoom_factor: f64 = 2.0_f64 / self.zoom;
        let x_min: f64 = zoom_factor.mul_add(-aspect_ratio, self.center.x);
        let x_max: f64 = zoom_factor.mul_add(aspect_ratio, self.center.x);
        let y_min: f64 = self.center.y - zoom_factor;
        let y_max: f64 = self.center.y + zoom_factor;

        let x_scale: f64 = (x_max - x_min) / f64::from(width);
        let y_scale: f64 = (y_max - y_min) / f64::from(height);

        (x_scale, y_scale, x_min, y_min)
    }

    /// Handles mouse input for zooming and panning the fractal view.
    #[inline]
    pub fn handle_mouse_input(&mut self, response: &egui::Response, image_rect: egui::Rect) {
        const ZOOM_STEP: f64 = 1.1;
        // Handle zoom with scroll wheel
        if response.hovered() {
            let scroll_delta = response.ctx.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                let zoom_factor = if scroll_delta > 0.0 {
                    ZOOM_STEP
                } else {
                    1.0 / ZOOM_STEP
                };

                // Get mouse position relative to image
                if let Some(mouse_pos) = response.ctx.input(|i| i.pointer.hover_pos()) {
                    let rel_pos = mouse_pos - image_rect.min;
                    let norm_x = rel_pos.x / image_rect.width();
                    let norm_y = rel_pos.y / image_rect.height();

                    // Convert normalized coordinates to complex plane
                    let aspect_ratio =
                        f64::from(image_rect.width()) / f64::from(image_rect.height());
                    let zoom_extent = 2.0 / self.zoom;

                    let mouse_complex_x = ((f64::from(norm_x) - 0.5) * zoom_extent * aspect_ratio)
                        .mul_add(2.0, self.center.x);
                    let mouse_complex_y =
                        ((f64::from(norm_y) - 0.5) * zoom_extent).mul_add(2.0, self.center.y);

                    // Zoom towards mouse position
                    let new_zoom = self.zoom * zoom_factor;
                    let new_zoom_extent = 2.0 / new_zoom;

                    // Adjust center to keep mouse position fixed
                    self.center.x = ((f64::from(norm_x) - 0.5) * new_zoom_extent * aspect_ratio)
                        .mul_add(-2.0, mouse_complex_x);
                    self.center.y = ((f64::from(norm_y) - 0.5) * new_zoom_extent)
                        .mul_add(-2.0, mouse_complex_y);

                    self.zoom = new_zoom;
                    self.needs_update = true;
                }
            }
        }

        // Handle panning with mouse drag
        if response.dragged() {
            let drag_delta = response.drag_delta();
            if response.drag_delta() != Vec2::ZERO {
                self.is_dragging = true;

                // Convert pixel drag to complex plane movement
                let aspect_ratio = f64::from(image_rect.width()) / f64::from(image_rect.height());
                let zoom_extent = 2.0 / self.zoom;

                let dx = -(f64::from(drag_delta.x) / f64::from(image_rect.width()))
                    * zoom_extent
                    * aspect_ratio
                    * 2.0;
                let dy =
                    -(f64::from(drag_delta.y) / f64::from(image_rect.height())) * zoom_extent * 2.0;

                self.center.x += dx;
                self.center.y += dy;
                self.needs_update = true;
            }
        } else {
            self.is_dragging = false;
        }

        // Handle double-click to zoom in
        if response.double_clicked()
            && let Some(click_pos) = response.interact_pointer_pos()
        {
            let rel_pos = click_pos - image_rect.min;
            let norm_x = rel_pos.x / image_rect.width();
            let norm_y = rel_pos.y / image_rect.height();

            // Convert to complex coordinates
            let aspect_ratio = f64::from(image_rect.width()) / f64::from(image_rect.height());
            let zoom_extent = 2.0_f64 / self.zoom;

            let new_center_x = ((f64::from(norm_x) - 0.5_f64) * zoom_extent * aspect_ratio)
                .mul_add(2.0_f64, self.center.x);
            let new_center_y =
                ((f64::from(norm_y) - 0.5_f64) * zoom_extent).mul_add(2.0_f64, self.center.y);

            self.center = Point::new(new_center_x, new_center_y);
            self.zoom *= 2.0_f64;
            self.needs_update = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_fractal_app() {
        let app = FractalApp::default();
        assert_eq!(app.fractal_type, FractalType::Mandelbrot);
        assert_eq!(app.max_iterations, 300);
        assert_eq!(app.center, Point::new(-0.5, 0.0));
        assert_eq!(app.zoom, 1.0);
        assert_eq!(app.julia_c, Point::new(-0.7269, 0.1889));
        assert!(app.needs_update);
        assert!(app.texture.is_none());
        assert_eq!(app.image_size, (800, 600));
        assert!(!app.is_dragging);
        assert!(!app.show_settings);
        assert_eq!(app.precision_mode, PrecisionMode::Fast);
        assert_eq!(app.color_scheme, ColorScheme::default());
    }
}
