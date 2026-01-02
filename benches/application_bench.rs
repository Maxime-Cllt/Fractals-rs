use criterion::{Criterion, criterion_group, criterion_main};
use fractals_rs::utils::point::Point;

// Test parameters
#[allow(unused)]
const TEST_X: f64 = -0.5;
#[allow(unused)]
const TEST_Y: f64 = 0.5;

#[allow(unused)]
const MAX_ITERATIONS: u16 = 1000;
#[allow(unused)]
fn benchmark_fractal_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("fractal_functions");

    let julia_c = Point::new(-0.7269, 0.1889);

    group.bench_function("mandelbrot", |b| {
        b.iter(|| std::hint::black_box(mandelbrot_iterations(TEST_X, TEST_Y, MAX_ITERATIONS)))
    });

    group.bench_function("julia", |b| {
        b.iter(|| std::hint::black_box(julia_iterations(TEST_X, TEST_Y, MAX_ITERATIONS, &julia_c)))
    });

    group.bench_function("burning_ship", |b| {
        b.iter(|| std::hint::black_box(burning_ship_iterations(TEST_X, TEST_Y, MAX_ITERATIONS)))
    });

    group.bench_function("tricorn", |b| {
        b.iter(|| std::hint::black_box(tricorn_iterations(TEST_X, TEST_Y, MAX_ITERATIONS)))
    });

    group.finish();
}

use fractals_rs::fractals::fractal_type::FractalType;
use fractals_rs::utils::precision_mode::PrecisionMode;

#[allow(unused)]
fn mandelbrot_iterations(x: f64, y: f64, max_iter: u16) -> u16 {
    FractalType::Mandelbrot.iterations(x, y, max_iter, &Point::new(0.0, 0.0), PrecisionMode::Fast)
}

#[allow(unused)]
fn julia_iterations(x: f64, y: f64, max_iter: u16, c: &Point) -> u16 {
    FractalType::Julia.iterations(x, y, max_iter, c, PrecisionMode::Fast)
}

#[allow(unused)]
fn burning_ship_iterations(x: f64, y: f64, max_iter: u16) -> u16 {
    FractalType::BurningShip.iterations(x, y, max_iter, &Point::new(0.0, 0.0), PrecisionMode::Fast)
}
#[allow(unused)]
fn tricorn_iterations(x: f64, y: f64, max_iter: u16) -> u16 {
    FractalType::Tricorn.iterations(x, y, max_iter, &Point::new(0.0, 0.0), PrecisionMode::Fast)
}

criterion_group!(benches, benchmark_fractal_functions);
criterion_main!(benches);
