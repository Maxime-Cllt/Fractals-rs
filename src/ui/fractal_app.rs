use crate::utils::precision_mode::PrecisionMode;
use crate::fractals::fractal_type::FractalType;
use crate::fractals::fractal_simd;
use crate::utils::color_scheme::ColorScheme;
use crate::utils::point::Point;
use egui::{Color32, Vec2};
use rayon::prelude::*;

/// Render all rows in parallel, dispatching to a single SIMD kernel.
/// The kernel closure is monomorphized per call site, allowing full inlining
/// of the SIMD body and eliminating the per-chunk fractal-type branch.
#[inline(always)]
fn render_image_simd_f32<K>(
    pixels: &mut [Color32],
    width: usize,
    x_min_f32: f32,
    y_min: f64,
    x_scale_f32: f32,
    y_scale: f64,
    max_iterations: u16,
    palette: &[Color32],
    kernel: K,
) where
    K: Fn(&[f32; 4], &[f32; 4], u16) -> [u16; 4] + Sync,
{
    // Lane offsets pre-computed once for the whole image — flat dependency chain
    // for each chunk's cx_arr (4 independent adds vs 3 sequential adds).
    let dx = x_scale_f32;
    let dx2 = dx + dx;
    let dx3 = dx2 + dx;
    let dx4 = dx2 + dx2;

    pixels
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row)| {
            let cy_f32 = (y as f64).mul_add(y_scale, y_min) as f32;
            let cy_arr = [cy_f32; 4];

            let mut cx_base = x_min_f32;
            let mut x = 0;

            // SIMD body: 4 pixels per call
            while x + 4 <= width {
                let cx_arr = [cx_base, cx_base + dx, cx_base + dx2, cx_base + dx3];
                let iters = kernel(&cx_arr, &cy_arr, max_iterations);
                row[x] = palette[iters[0] as usize];
                row[x + 1] = palette[iters[1] as usize];
                row[x + 2] = palette[iters[2] as usize];
                row[x + 3] = palette[iters[3] as usize];
                cx_base += dx4;
                x += 4;
            }

            // Tail: fewer than 4 pixels — still go through the same kernel so we
            // don't pay a separate scalar code path. Lanes past the row width
            // get duplicate inputs; we only consume `iters[0]`.
            while x < width {
                let cx_arr = [cx_base; 4];
                let iters = kernel(&cx_arr, &cy_arr, max_iterations);
                row[x] = palette[iters[0] as usize];
                cx_base += dx;
                x += 1;
            }
        });
}

/// Render all rows in parallel for f64 precision.
#[inline(always)]
fn render_image_simd_f64<K>(
    pixels: &mut [Color32],
    width: usize,
    x_min: f64,
    y_min: f64,
    x_scale: f64,
    y_scale: f64,
    max_iterations: u16,
    palette: &[Color32],
    kernel: K,
) where
    K: Fn(&[f64; 2], &[f64; 2], u16) -> [u16; 2] + Sync,
{
    let dx = x_scale;
    let dx2 = dx + dx;

    pixels
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row)| {
            let cy = (y as f64).mul_add(y_scale, y_min);
            let cy_arr = [cy; 2];

            let mut cx_base = x_min;
            let mut x = 0;

            while x + 2 <= width {
                let cx_arr = [cx_base, cx_base + dx];
                let iters = kernel(&cx_arr, &cy_arr, max_iterations);
                row[x] = palette[iters[0] as usize];
                row[x + 1] = palette[iters[1] as usize];
                cx_base += dx2;
                x += 2;
            }

            while x < width {
                let cx_arr = [cx_base; 2];
                let iters = kernel(&cx_arr, &cy_arr, max_iterations);
                row[x] = palette[iters[0] as usize];
                cx_base += dx;
                x += 1;
            }
        });
}

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
    /// Highly optimized with:
    /// - Row-based parallelization
    /// - SIMD vectorization (4x f32 or 2x f64 pixels per operation)
    /// - Direct RGBA buffer generation
    /// - FMA operations and loop unrolling
    #[inline]
    #[must_use]
    pub fn generate_fractal_image(&self) -> egui::ColorImage {
        let width = self.image_size.0 as usize;
        let height = self.image_size.1 as usize;

        if width == 0 || height == 0 {
            return egui::ColorImage::new([1, 1], vec![Color32::BLACK; 1]);
        }

        let (x_scale, y_scale, x_min, y_min) = self.compute_scale();
        let total_pixels = width * height;

        // Pre-compute color palette once — avoids heavy trig per pixel
        let palette = self.color_scheme.build_palette(self.max_iterations);

        // Build Vec<Color32> directly — skips the rgba_buffer allocation and
        // the from_rgba_unmultiplied copy at the end (saves ~1 full image copy)
        let mut pixels = vec![Color32::BLACK; total_pixels];

        let max_iterations = self.max_iterations;
        let julia_cx_f32 = self.julia_c.x as f32;
        let julia_cy_f32 = self.julia_c.y as f32;
        let julia_cx = self.julia_c.x;
        let julia_cy = self.julia_c.y;

        // Dispatch once per frame, not per chunk-of-4 pixels: this lets each
        // (fractal_type, precision_mode) combo be fully monomorphized so the
        // SIMD kernel inlines into the row loop without any per-chunk branch.
        match self.precision_mode {
            PrecisionMode::Fast => {
                let x_scale_f32 = x_scale as f32;
                let x_min_f32 = x_min as f32;
                match self.fractal_type {
                    FractalType::Mandelbrot => render_image_simd_f32(
                        &mut pixels, width, x_min_f32, y_min, x_scale_f32, y_scale,
                        max_iterations, &palette,
                        |cx, cy, mi| fractal_simd::mandelbrot_simd_f32(cx, cy, mi),
                    ),
                    FractalType::Julia => render_image_simd_f32(
                        &mut pixels, width, x_min_f32, y_min, x_scale_f32, y_scale,
                        max_iterations, &palette,
                        |cx, cy, mi| fractal_simd::julia_simd_f32(cx, cy, julia_cx_f32, julia_cy_f32, mi),
                    ),
                    FractalType::BurningShip => render_image_simd_f32(
                        &mut pixels, width, x_min_f32, y_min, x_scale_f32, y_scale,
                        max_iterations, &palette,
                        |cx, cy, mi| fractal_simd::burning_ship_simd_f32(cx, cy, mi),
                    ),
                    FractalType::Tricorn => render_image_simd_f32(
                        &mut pixels, width, x_min_f32, y_min, x_scale_f32, y_scale,
                        max_iterations, &palette,
                        |cx, cy, mi| fractal_simd::tricorn_simd_f32(cx, cy, mi),
                    ),
                }
            }
            PrecisionMode::High => match self.fractal_type {
                FractalType::Mandelbrot => render_image_simd_f64(
                    &mut pixels, width, x_min, y_min, x_scale, y_scale,
                    max_iterations, &palette,
                    |cx, cy, mi| fractal_simd::mandelbrot_simd_f64(cx, cy, mi),
                ),
                FractalType::Julia => render_image_simd_f64(
                    &mut pixels, width, x_min, y_min, x_scale, y_scale,
                    max_iterations, &palette,
                    |cx, cy, mi| fractal_simd::julia_simd_f64(cx, cy, julia_cx, julia_cy, mi),
                ),
                FractalType::BurningShip => render_image_simd_f64(
                    &mut pixels, width, x_min, y_min, x_scale, y_scale,
                    max_iterations, &palette,
                    |cx, cy, mi| fractal_simd::burning_ship_simd_f64(cx, cy, mi),
                ),
                FractalType::Tricorn => render_image_simd_f64(
                    &mut pixels, width, x_min, y_min, x_scale, y_scale,
                    max_iterations, &palette,
                    |cx, cy, mi| fractal_simd::tricorn_simd_f64(cx, cy, mi),
                ),
            },
            #[cfg(feature = "f128")]
            PrecisionMode::UltraHigh => {
                let fractal_type = self.fractal_type;
                let precision_mode = self.precision_mode;
                let julia_c = &self.julia_c;
                pixels
                    .par_chunks_mut(width)
                    .enumerate()
                    .for_each(|(y, row)| {
                        let cy = (y as f64).mul_add(y_scale, y_min);
                        let mut cx = x_min;
                        for pixel in row.iter_mut() {
                            let iterations = fractal_type.iterations(
                                cx,
                                cy,
                                max_iterations,
                                julia_c,
                                precision_mode,
                            );
                            *pixel = palette[iterations as usize];
                            cx += x_scale;
                        }
                    });
            }
        }

        // Construct ColorImage directly from Vec<Color32> — no extra allocation/copy
        egui::ColorImage {
            size: [width, height],
            source_size: egui::Vec2::new(width as f32, height as f32),
            pixels,
        }
    }

    /// Computes the scale factors and min/max coordinates for the fractal view.
    #[inline(always)]
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
