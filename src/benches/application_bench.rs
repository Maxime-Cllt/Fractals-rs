// use fractals_rs::fractals::FractalType;
// use fractals_rs::point::Point;
// use criterion::{criterion_group, criterion_main, Criterion};
// 
// fn test_compute_mandelbrot() {
//     const WIDTH: usize = 800;
//     const HEIGHT: usize = 600;
// 
//     const MAX_ITERATIONS: u16 = 100;
// 
//     const JULIA_C: Point = Point::new(-0.7269, 0.1889);
// 
//     let x_min = -2.0;
//     let x_max = 1.0;
//     let y_min = -1.0;
//     let y_max = 1.0;
// 
//     let x_scale = (x_max - x_min) / WIDTH as f64;
//     let y_scale = (y_max - y_min) / HEIGHT as f64;
// 
//     for y in 0..HEIGHT {
//         for x in 0..WIDTH {
//             let cx = x_min + x as f64 * x_scale;
//             let cy = y_min + y as f64 * y_scale;
//             FractalType::Mandelbrot.iterations(cx, cy, MAX_ITERATIONS, &JULIA_C);
//         }
//     }
// }
// 
// fn benchmark_application(c: &mut Criterion) {
//     let mut group = c.benchmark_group("benchmark_application");
// 
//     group.bench_function("test_compute_mandelbrot", |b| {
//         b.iter(|| {
//             test_compute_mandelbrot();
//         })
//     });
// 
//     group.finish();
// }
// 
// criterion_group!(benches, benchmark_application);
// criterion_main!(benches);
