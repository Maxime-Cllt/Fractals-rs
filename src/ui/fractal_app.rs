use crate::enums::fractal_type::FractalType;
use crate::structs::color_scheme::ColorScheme;
use crate::structs::fractal_app::FractalApp;
use crate::structs::point::Point;
use egui::{Color32, Vec2};
use rayon::prelude::*;
use crate::enums::precision_mode::PrecisionMode;

impl Default for FractalApp {
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
                    let cx = x_min + x as f64 * x_scale;
                    let cy = y_min + y as f64 * y_scale;

                    let iterations = self.fractal_type.iterations(
                        cx, cy, self.max_iterations, &self.julia_c, self.precision_mode
                    );

                    self.color_scheme.to_color32(iterations, self.max_iterations)
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

    fn compute_scale(&self) -> (f64, f64, f64, f64) {
        let width: u32 = self.image_size.0;
        let height: u32 = self.image_size.1;

        let aspect_ratio: f64 = width as f64 / height as f64;
        let zoom_factor: f64 = 2.0 / self.zoom;
        let x_min: f64 = self.center.x - zoom_factor * aspect_ratio;
        let x_max: f64 = self.center.x + zoom_factor * aspect_ratio;
        let y_min: f64 = self.center.y - zoom_factor;
        let y_max: f64 = self.center.y + zoom_factor;

        let x_scale: f64 = (x_max - x_min) / width as f64;
        let y_scale: f64 = (y_max - y_min) / height as f64;

        (x_scale, y_scale, x_min, y_min)
    }

    pub fn handle_mouse_input(&mut self, response: &egui::Response, image_rect: egui::Rect) {
        // Handle zoom with scroll wheel
        if response.hovered() {
            let scroll_delta = response.ctx.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 1.0 / 1.1 };

                // Get mouse position relative to image
                if let Some(mouse_pos) = response.ctx.input(|i| i.pointer.hover_pos()) {
                    let rel_pos = mouse_pos - image_rect.min;
                    let norm_x = rel_pos.x / image_rect.width();
                    let norm_y = rel_pos.y / image_rect.height();

                    // Convert normalized coordinates to complex plane
                    let aspect_ratio = image_rect.width() as f64 / image_rect.height() as f64;
                    let zoom_extent = 2.0 / self.zoom;

                    let mouse_complex_x =
                        self.center.x + (norm_x as f64 - 0.5) * zoom_extent * aspect_ratio * 2.0;
                    let mouse_complex_y = self.center.y + (norm_y as f64 - 0.5) * zoom_extent * 2.0;

                    // Zoom towards mouse position
                    let new_zoom = self.zoom * zoom_factor;
                    let new_zoom_extent = 2.0 / new_zoom;

                    // Adjust center to keep mouse position fixed
                    self.center.x = mouse_complex_x
                        - (norm_x as f64 - 0.5) * new_zoom_extent * aspect_ratio * 2.0;
                    self.center.y = mouse_complex_y - (norm_y as f64 - 0.5) * new_zoom_extent * 2.0;

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
                let aspect_ratio = image_rect.width() as f64 / image_rect.height() as f64;
                let zoom_extent = 2.0 / self.zoom;

                let dx = -(drag_delta.x as f64 / image_rect.width() as f64)
                    * zoom_extent
                    * aspect_ratio
                    * 2.0;
                let dy = -(drag_delta.y as f64 / image_rect.height() as f64) * zoom_extent * 2.0;

                self.center.x += dx;
                self.center.y += dy;
                self.needs_update = true;
            }
        } else {
            self.is_dragging = false;
        }

        // Handle double-click to zoom in
        if response.double_clicked() {
            if let Some(click_pos) = response.interact_pointer_pos() {
                let rel_pos = click_pos - image_rect.min;
                let norm_x = rel_pos.x / image_rect.width();
                let norm_y = rel_pos.y / image_rect.height();

                // Convert to complex coordinates
                let aspect_ratio = image_rect.width() as f64 / image_rect.height() as f64;
                let zoom_extent = 2.0 / self.zoom;

                let new_center_x =
                    self.center.x + (norm_x as f64 - 0.5) * zoom_extent * aspect_ratio * 2.0;
                let new_center_y = self.center.y + (norm_y as f64 - 0.5) * zoom_extent * 2.0;

                self.center = Point::new(new_center_x, new_center_y);
                self.zoom *= 2.0;
                self.needs_update = true;
            }
        }
    }
}
