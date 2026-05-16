use criterion::{Criterion, criterion_group, criterion_main};
use fractals_rs::fractals::fractal_simd;
use fractals_rs::fractals::fractal_type::FractalType;
use fractals_rs::utils::color_scheme::ColorScheme;
use fractals_rs::utils::point::Point;
use fractals_rs::utils::precision_mode::PrecisionMode;

// Test parameters
const TEST_X: f64 = -0.5;
const TEST_Y: f64 = 0.5;
const MAX_ITERATIONS: u16 = 1000;

fn benchmark_fractal_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_kernels");

    let julia_c = Point::new(-0.7269, 0.1889);

    group.bench_function("mandelbrot_f32", |b| {
        b.iter(|| std::hint::black_box(mandelbrot_iterations(TEST_X, TEST_Y, MAX_ITERATIONS)))
    });

    group.bench_function("julia_f32", |b| {
        b.iter(|| std::hint::black_box(julia_iterations(TEST_X, TEST_Y, MAX_ITERATIONS, &julia_c)))
    });

    group.bench_function("burning_ship_f32", |b| {
        b.iter(|| std::hint::black_box(burning_ship_iterations(TEST_X, TEST_Y, MAX_ITERATIONS)))
    });

    group.bench_function("tricorn_f32", |b| {
        b.iter(|| std::hint::black_box(tricorn_iterations(TEST_X, TEST_Y, MAX_ITERATIONS)))
    });

    group.finish();
}

fn benchmark_simd_kernels(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_kernels");

    let cx = [-0.5_f32, 0.25, -1.0, 0.1];
    let cy = [0.5_f32, -0.5, 0.0, 0.25];
    let cx64 = [-0.5_f64, 0.25];
    let cy64 = [0.5_f64, -0.5];

    group.bench_function("mandelbrot_simd_f32", |b| {
        b.iter(|| std::hint::black_box(fractal_simd::mandelbrot_simd_f32(&cx, &cy, MAX_ITERATIONS)))
    });
    group.bench_function("julia_simd_f32", |b| {
        b.iter(|| {
            std::hint::black_box(fractal_simd::julia_simd_f32(
                &cx,
                &cy,
                -0.7269,
                0.1889,
                MAX_ITERATIONS,
            ))
        })
    });
    group.bench_function("burning_ship_simd_f32", |b| {
        b.iter(|| std::hint::black_box(fractal_simd::burning_ship_simd_f32(&cx, &cy, MAX_ITERATIONS)))
    });
    group.bench_function("tricorn_simd_f32", |b| {
        b.iter(|| std::hint::black_box(fractal_simd::tricorn_simd_f32(&cx, &cy, MAX_ITERATIONS)))
    });

    group.bench_function("mandelbrot_simd_f64", |b| {
        b.iter(|| std::hint::black_box(fractal_simd::mandelbrot_simd_f64(&cx64, &cy64, MAX_ITERATIONS)))
    });

    group.finish();
}

/// Synthetic full-frame render: this is the metric that matters most for UX
/// (time to produce one frame at a typical zoom).
fn benchmark_full_frame(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_frame_800x600");
    group.sample_size(20);

    let width = 800usize;
    let height = 600usize;
    let max_iter: u16 = 300;
    let palette = ColorScheme::default().build_palette(max_iter);

    // Mandelbrot default view
    let center = Point::new(-0.5, 0.0);
    let zoom = 1.0_f64;
    let aspect = width as f64 / height as f64;
    let zoom_factor = 2.0 / zoom;
    let x_min = center.x - zoom_factor * aspect;
    let x_max = center.x + zoom_factor * aspect;
    let y_min = center.y - zoom_factor;
    let y_max = center.y + zoom_factor;
    let x_scale = (x_max - x_min) / width as f64;
    let y_scale = (y_max - y_min) / height as f64;

    group.bench_function("mandelbrot_fast", |b| {
        b.iter(|| {
            let mut pixels = vec![eframe::epaint::Color32::BLACK; width * height];
            render_frame_f32(
                &mut pixels,
                width,
                height,
                x_min as f32,
                y_min,
                x_scale as f32,
                y_scale,
                max_iter,
                &palette,
                |cx, cy, mi| fractal_simd::mandelbrot_simd_f32(cx, cy, mi),
            );
            std::hint::black_box(pixels)
        })
    });

    group.bench_function("julia_fast", |b| {
        b.iter(|| {
            let mut pixels = vec![eframe::epaint::Color32::BLACK; width * height];
            render_frame_f32(
                &mut pixels,
                width,
                height,
                x_min as f32,
                y_min,
                x_scale as f32,
                y_scale,
                max_iter,
                &palette,
                |cx, cy, mi| fractal_simd::julia_simd_f32(cx, cy, -0.7269, 0.1889, mi),
            );
            std::hint::black_box(pixels)
        })
    });

    group.bench_function("burning_ship_fast", |b| {
        b.iter(|| {
            let mut pixels = vec![eframe::epaint::Color32::BLACK; width * height];
            render_frame_f32(
                &mut pixels,
                width,
                height,
                x_min as f32,
                y_min,
                x_scale as f32,
                y_scale,
                max_iter,
                &palette,
                |cx, cy, mi| fractal_simd::burning_ship_simd_f32(cx, cy, mi),
            );
            std::hint::black_box(pixels)
        })
    });

    group.bench_function("mandelbrot_high", |b| {
        b.iter(|| {
            let mut pixels = vec![eframe::epaint::Color32::BLACK; width * height];
            render_frame_f64(
                &mut pixels,
                width,
                height,
                x_min,
                y_min,
                x_scale,
                y_scale,
                max_iter,
                &palette,
                |cx, cy, mi| fractal_simd::mandelbrot_simd_f64(cx, cy, mi),
            );
            std::hint::black_box(pixels)
        })
    });

    group.finish();
}

fn render_frame_f32<K>(
    pixels: &mut [eframe::epaint::Color32],
    width: usize,
    _height: usize,
    x_min_f32: f32,
    y_min: f64,
    x_scale_f32: f32,
    y_scale: f64,
    max_iterations: u16,
    palette: &[eframe::epaint::Color32],
    kernel: K,
) where
    K: Fn(&[f32; 4], &[f32; 4], u16) -> [u16; 4] + Sync,
{
    use rayon::prelude::*;

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

            while x < width {
                let cx_arr = [cx_base; 4];
                let iters = kernel(&cx_arr, &cy_arr, max_iterations);
                row[x] = palette[iters[0] as usize];
                cx_base += dx;
                x += 1;
            }
        });
}

fn render_frame_f64<K>(
    pixels: &mut [eframe::epaint::Color32],
    width: usize,
    _height: usize,
    x_min: f64,
    y_min: f64,
    x_scale: f64,
    y_scale: f64,
    max_iterations: u16,
    palette: &[eframe::epaint::Color32],
    kernel: K,
) where
    K: Fn(&[f64; 2], &[f64; 2], u16) -> [u16; 2] + Sync,
{
    use rayon::prelude::*;

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

fn mandelbrot_iterations(x: f64, y: f64, max_iter: u16) -> u16 {
    FractalType::Mandelbrot.iterations(x, y, max_iter, &Point::new(0.0, 0.0), PrecisionMode::Fast)
}

fn julia_iterations(x: f64, y: f64, max_iter: u16, c: &Point) -> u16 {
    FractalType::Julia.iterations(x, y, max_iter, c, PrecisionMode::Fast)
}

fn burning_ship_iterations(x: f64, y: f64, max_iter: u16) -> u16 {
    FractalType::BurningShip.iterations(x, y, max_iter, &Point::new(0.0, 0.0), PrecisionMode::Fast)
}

fn tricorn_iterations(x: f64, y: f64, max_iter: u16) -> u16 {
    FractalType::Tricorn.iterations(x, y, max_iter, &Point::new(0.0, 0.0), PrecisionMode::Fast)
}

criterion_group!(
    benches,
    benchmark_fractal_functions,
    benchmark_simd_kernels,
    benchmark_full_frame
);
criterion_main!(benches);
