/// Optimized fractal computation kernels with direct f32/f64 implementations.
/// This module replaces the trait-based abstraction for maximum performance.
use crate::utils::point::Point;

#[cfg(feature = "f128")]
use rust_decimal::Decimal;
#[cfg(feature = "f128")]
use rust_decimal_macros::dec;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Check if a point is in the main cardioid or period-2 bulb of the Mandelbrot set.
/// This early-out optimization skips ~25% of calculations.
#[inline(always)]
fn mandelbrot_early_out_f32(cx: f32, cy: f32) -> bool {
    // Check main cardioid: q*(q + (x-0.25)) < 0.25*y^2 where q = (x-0.25)^2 + y^2
    let x_offset = cx - 0.25;
    let q = x_offset.mul_add(x_offset, cy * cy);
    if q.mul_add(q + x_offset, -(0.25 * cy * cy)) < 0.0 {
        return true;
    }

    // Check period-2 bulb: (x+1)^2 + y^2 < 0.0625
    let x_plus_one = cx + 1.0;
    if x_plus_one.mul_add(x_plus_one, cy * cy) < 0.0625 {
        return true;
    }

    false
}

#[inline(always)]
fn mandelbrot_early_out_f64(cx: f64, cy: f64) -> bool {
    let x_offset = cx - 0.25;
    let q = x_offset.mul_add(x_offset, cy * cy);
    if q.mul_add(q + x_offset, -(0.25 * cy * cy)) < 0.0 {
        return true;
    }

    let x_plus_one = cx + 1.0;
    if x_plus_one.mul_add(x_plus_one, cy * cy) < 0.0625 {
        return true;
    }

    false
}

// ============================================================================
// MANDELBROT KERNELS
// ============================================================================

/// Highly optimized Mandelbrot iteration kernel for f32.
/// Features: FMA operations, loop unrolling, early bailout, cardioid checking.
#[inline(always)]
pub fn mandelbrot_iterations_f32(cx: f32, cy: f32, max_iteration: u16) -> u16 {
    // Early exit for points in main set components
    if mandelbrot_early_out_f32(cx, cy) {
        return max_iteration;
    }

    let mut zr = 0.0f32;
    let mut zi = 0.0f32;
    let mut iterations = 0u16;

    // Manual loop unrolling: 4 iterations per loop
    // This improves ILP (instruction-level parallelism) and reduces branch overhead
    while iterations + 4 <= max_iteration {
        // Iteration 1
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;

        // Iteration 2
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;

        // Iteration 3
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;

        // Iteration 4
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;
    }

    // Handle remaining iterations (< 4)
    while iterations < max_iteration {
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;
    }

    iterations
}

/// Highly optimized Mandelbrot iteration kernel for f64.
#[inline(always)]
pub fn mandelbrot_iterations_f64(cx: f64, cy: f64, max_iteration: u16) -> u16 {
    if mandelbrot_early_out_f64(cx, cy) {
        return max_iteration;
    }

    let mut zr = 0.0f64;
    let mut zi = 0.0f64;
    let mut iterations = 0u16;

    // Manual loop unrolling for f64
    while iterations + 4 <= max_iteration {
        // Iteration 1
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;

        // Iteration 2
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;

        // Iteration 3
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;

        // Iteration 4
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;
    }

    while iterations < max_iteration {
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > 4.0 {
            break;
        }
        let new_zr = zr2.mul_add(1.0, zi2.mul_add(-1.0, cx));
        zi = (2.0 * zr).mul_add(zi, cy);
        zr = new_zr;
        iterations += 1;
    }

    iterations
}

// ============================================================================
// JULIA KERNELS
// ============================================================================

/// Optimized Julia set iteration kernel for f32.
#[inline(always)]
pub fn julia_iterations_f32(zx: f32, zy: f32, max_iteration: u16, c: &Point) -> u16 {
    let mut x = zx;
    let mut y = zy;
    let mut iterations = 0u16;
    let cx = c.x as f32;
    let cy = c.y as f32;

    // Manual loop unrolling
    while iterations + 4 <= max_iteration {
        // Iteration 1
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;

        // Iteration 2
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;

        // Iteration 3
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;

        // Iteration 4
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;
    }

    while iterations < max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;
    }

    iterations
}

/// Optimized Julia set iteration kernel for f64.
#[inline(always)]
pub fn julia_iterations_f64(zx: f64, zy: f64, max_iteration: u16, c: &Point) -> u16 {
    let mut x = zx;
    let mut y = zy;
    let mut iterations = 0u16;
    let cx = c.x;
    let cy = c.y;

    while iterations + 4 <= max_iteration {
        // Iteration 1
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;

        // Iteration 2
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;

        // Iteration 3
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;

        // Iteration 4
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;
    }

    while iterations < max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let new_y = (2.0 * x).mul_add(y, cy);
        x = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = new_y;
        iterations += 1;
    }

    iterations
}

// ============================================================================
// BURNING SHIP KERNELS
// ============================================================================

/// Optimized Burning Ship iteration kernel for f32.
#[inline(always)]
pub fn burning_ship_iterations_f32(cx: f32, cy: f32, max_iteration: u16) -> u16 {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut iterations = 0u16;

    // Manual loop unrolling
    while iterations + 4 <= max_iteration {
        // Iteration 1
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;

        // Iteration 2
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;

        // Iteration 3
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;

        // Iteration 4
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;
    }

    while iterations < max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;
    }

    iterations
}

/// Optimized Burning Ship iteration kernel for f64.
#[inline(always)]
pub fn burning_ship_iterations_f64(cx: f64, cy: f64, max_iteration: u16) -> u16 {
    let mut x = 0.0f64;
    let mut y = 0.0f64;
    let mut iterations = 0u16;

    while iterations + 4 <= max_iteration {
        // Iteration 1
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;

        // Iteration 2
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;

        // Iteration 3
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;

        // Iteration 4
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;
    }

    while iterations < max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (2.0 * x.abs()).mul_add(y.abs(), cy);
        x = temp;
        iterations += 1;
    }

    iterations
}

// ============================================================================
// TRICORN KERNELS
// ============================================================================

/// Optimized Tricorn iteration kernel for f32.
#[inline(always)]
pub fn tricorn_iterations_f32(cx: f32, cy: f32, max_iteration: u16) -> u16 {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut iterations = 0u16;

    // Manual loop unrolling
    while iterations + 4 <= max_iteration {
        // Iteration 1
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;

        // Iteration 2
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;

        // Iteration 3
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;

        // Iteration 4
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;
    }

    while iterations < max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;
    }

    iterations
}

/// Optimized Tricorn iteration kernel for f64.
#[inline(always)]
pub fn tricorn_iterations_f64(cx: f64, cy: f64, max_iteration: u16) -> u16 {
    let mut x = 0.0f64;
    let mut y = 0.0f64;
    let mut iterations = 0u16;

    while iterations + 4 <= max_iteration {
        // Iteration 1
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;

        // Iteration 2
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;

        // Iteration 3
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;

        // Iteration 4
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;
    }

    while iterations < max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            break;
        }
        let temp = x2.mul_add(1.0, y2.mul_add(-1.0, cx));
        y = (-2.0 * x).mul_add(y, cy);
        x = temp;
        iterations += 1;
    }

    iterations
}

// ============================================================================
// F128 (DECIMAL) KERNELS - Ultra High Precision
// ============================================================================

#[cfg(feature = "f128")]
/// Check if a point is in the main cardioid or period-2 bulb of the Mandelbrot set (f128 version).
#[inline(always)]
fn mandelbrot_early_out_f128(cx: Decimal, cy: Decimal) -> bool {
    let quarter = dec!(0.25);
    let one = Decimal::ONE;
    let x_offset = cx - quarter;
    let q = x_offset * x_offset + cy * cy;
    if q * (q + x_offset) < quarter * cy * cy {
        return true;
    }

    let x_plus_one = cx + one;
    if x_plus_one * x_plus_one + cy * cy < dec!(0.0625) {
        return true;
    }

    false
}

#[cfg(feature = "f128")]
/// Mandelbrot iteration kernel for f128 (Decimal) precision.
/// Uses 128-bit decimal arithmetic for extreme zoom levels.
#[inline(always)]
pub fn mandelbrot_iterations_f128(cx: Decimal, cy: Decimal, max_iteration: u16) -> u16 {
    if mandelbrot_early_out_f128(cx, cy) {
        return max_iteration;
    }

    let mut zr = Decimal::ZERO;
    let mut zi = Decimal::ZERO;
    let mut iterations = 0u16;
    let four = dec!(4);

    while iterations < max_iteration {
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if zr2 + zi2 > four {
            break;
        }
        let new_zr = zr2 - zi2 + cx;
        zi = dec!(2) * zr * zi + cy;
        zr = new_zr;
        iterations += 1;
    }

    iterations
}

#[cfg(feature = "f128")]
/// Julia set iteration kernel for f128 (Decimal) precision.
#[inline(always)]
pub fn julia_iterations_f128(zx: Decimal, zy: Decimal, max_iteration: u16, c: &Point) -> u16 {
    let mut x = zx;
    let mut y = zy;
    let mut iterations = 0u16;
    let cx = Decimal::from_f64_retain(c.x).unwrap_or(Decimal::ZERO);
    let cy = Decimal::from_f64_retain(c.y).unwrap_or(Decimal::ZERO);
    let four = dec!(4);

    while iterations < max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > four {
            break;
        }
        let new_y = dec!(2) * x * y + cy;
        x = x2 - y2 + cx;
        y = new_y;
        iterations += 1;
    }

    iterations
}

#[cfg(feature = "f128")]
/// Burning Ship iteration kernel for f128 (Decimal) precision.
#[inline(always)]
pub fn burning_ship_iterations_f128(cx: Decimal, cy: Decimal, max_iteration: u16) -> u16 {
    let mut x = Decimal::ZERO;
    let mut y = Decimal::ZERO;
    let mut iterations = 0u16;
    let four = dec!(4);

    while iterations < max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > four {
            break;
        }
        let temp = x2 - y2 + cx;
        y = dec!(2) * x.abs() * y.abs() + cy;
        x = temp;
        iterations += 1;
    }

    iterations
}

#[cfg(feature = "f128")]
/// Tricorn iteration kernel for f128 (Decimal) precision.
#[inline(always)]
pub fn tricorn_iterations_f128(cx: Decimal, cy: Decimal, max_iteration: u16) -> u16 {
    let mut x = Decimal::ZERO;
    let mut y = Decimal::ZERO;
    let mut iterations = 0u16;
    let four = dec!(4);

    while iterations < max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > four {
            break;
        }
        let temp = x2 - y2 + cx;
        y = dec!(-2) * x * y + cy;
        x = temp;
        iterations += 1;
    }

    iterations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandelbrot_f32() {
        let iterations = mandelbrot_iterations_f32(0.0, 0.0, 1000);
        assert!(iterations > 0);
    }

    #[test]
    fn test_mandelbrot_f64() {
        let iterations = mandelbrot_iterations_f64(0.0, 0.0, 1000);
        assert!(iterations > 0);
    }

    #[test]
    fn test_mandelbrot_early_out() {
        // Point in main cardioid should return max_iterations
        let iterations = mandelbrot_iterations_f32(-0.5, 0.0, 100);
        assert_eq!(iterations, 100);
    }

    #[test]
    fn test_julia_f32() {
        let c = Point::new(0.355, 0.355);
        let iterations = julia_iterations_f32(0.0, 0.0, 1000, &c);
        assert!(iterations > 0);
    }

    #[test]
    fn test_burning_ship_f32() {
        let iterations = burning_ship_iterations_f32(0.0, 0.0, 1000);
        assert!(iterations > 0);
    }

    #[test]
    fn test_tricorn_f32() {
        let iterations = tricorn_iterations_f32(0.0, 0.0, 1000);
        assert!(iterations > 0);
    }

    #[cfg(feature = "f128")]
    #[test]
    fn test_mandelbrot_f128() {
        use rust_decimal::Decimal;
        let iterations = mandelbrot_iterations_f128(Decimal::ZERO, Decimal::ZERO, 1000);
        assert!(iterations > 0);
    }

    #[cfg(feature = "f128")]
    #[test]
    fn test_julia_f128() {
        use rust_decimal::Decimal;
        let c = Point::new(0.355, 0.355);
        let iterations = julia_iterations_f128(Decimal::ZERO, Decimal::ZERO, 1000, &c);
        assert!(iterations > 0);
    }

    #[cfg(feature = "f128")]
    #[test]
    fn test_burning_ship_f128() {
        use rust_decimal::Decimal;
        let iterations = burning_ship_iterations_f128(Decimal::ZERO, Decimal::ZERO, 1000);
        assert!(iterations > 0);
    }

    #[cfg(feature = "f128")]
    #[test]
    fn test_tricorn_f128() {
        use rust_decimal::Decimal;
        let iterations = tricorn_iterations_f128(Decimal::ZERO, Decimal::ZERO, 1000);
        assert!(iterations > 0);
    }
}
