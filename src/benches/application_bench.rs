use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fractals_rs::structs::point::Point;

// Test parameters
const TEST_X: f64 = -0.5;
const TEST_Y: f64 = 0.5;
const MAX_ITERATIONS: u16 = 1000;

fn benchmark_fractal_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("fractal_functions");

    let julia_c = Point::new(-0.7269, 0.1889);

    group.bench_function("mandelbrot", |b| {
        b.iter(|| {
            std::hint::black_box(mandelbrot_iterations(TEST_X, TEST_Y, MAX_ITERATIONS))
        })
    });

    group.bench_function("julia", |b| {
        b.iter(|| {
            std::hint::black_box(julia_iterations(TEST_X, TEST_Y, MAX_ITERATIONS, &julia_c))
        })
    });

    group.bench_function("burning_ship", |b| {
        b.iter(|| {
            std::hint::black_box(burning_ship_iterations(TEST_X, TEST_Y, MAX_ITERATIONS))
        })
    });

    group.bench_function("tricorn", |b| {
        b.iter(|| {
            std::hint::black_box(tricorn_iterations(TEST_X, TEST_Y, MAX_ITERATIONS))
        })
    });

    group.finish();
}

use fractals_rs::enums::fractal_type::FractalType;

fn mandelbrot_iterations(x: f64, y: f64, max_iter: u16) -> u16 {
    FractalType::Mandelbrot.iterations(x, y, max_iter, &Point::new(0.0, 0.0))
}

fn julia_iterations(x: f64, y: f64, max_iter: u16, c: &Point) -> u16 {
    FractalType::Julia.iterations(x, y, max_iter, c)
}

fn burning_ship_iterations(x: f64, y: f64, max_iter: u16) -> u16 {
    FractalType::BurningShip.iterations(x, y, max_iter, &Point::new(0.0, 0.0))
}

fn tricorn_iterations(x: f64, y: f64, max_iter: u16) -> u16 {
    FractalType::Tricorn.iterations(x, y, max_iter, &Point::new(0.0, 0.0))
}

criterion_group!(benches, benchmark_fractal_functions);
criterion_main!(benches);