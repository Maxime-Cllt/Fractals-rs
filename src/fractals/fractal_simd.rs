/// SIMD-accelerated fractal computation kernels.
/// Uses the `wide` crate for portable SIMD operations across platforms.
///
/// Key optimizations:
/// - Process 4 pixels simultaneously with f32x4 (SSE/AVX)
/// - Process 2 pixels simultaneously with f64x2
/// - Vectorized escape-time algorithm
/// - Early termination with active masks
use wide::{f32x4, f64x2};

// ============================================================================
// MANDELBROT SIMD KERNELS
// ============================================================================

/// SIMD Mandelbrot kernel processing 4 f32 pixels simultaneously.
///
/// # Arguments
/// * `cx` - Array of 4 x-coordinates
/// * `cy` - Array of 4 y-coordinates
/// * `max_iteration` - Maximum iteration count
///
/// # Returns
/// Array of 4 iteration counts
#[inline(always)]
pub fn mandelbrot_simd_f32(cx: &[f32; 4], cy: &[f32; 4], max_iteration: u16) -> [u16; 4] {
    let mut iterations = [0u16; 4];
    let mut active_mask = [true; 4];

    // Early exit checks for each pixel
    for i in 0..4 {
        let x_offset = cx[i] - 0.25;
        let q = x_offset * x_offset + cy[i] * cy[i];
        if q * (q + x_offset) < 0.25 * cy[i] * cy[i] {
            iterations[i] = max_iteration;
            active_mask[i] = false;
            continue;
        }
        let x_plus = cx[i] + 1.0;
        if x_plus * x_plus + cy[i] * cy[i] < 0.0625 {
            iterations[i] = max_iteration;
            active_mask[i] = false;
        }
    }

    if !active_mask.iter().any(|&b| b) {
        return iterations;
    }

    let cx_vec = f32x4::from(*cx);
    let cy_vec = f32x4::from(*cy);

    let mut zr = f32x4::ZERO;
    let mut zi = f32x4::ZERO;
    let two = f32x4::splat(2.0);

    for iter in 0..max_iteration {
        if !active_mask.iter().any(|&b| b) {
            break;
        }

        let zr2 = zr * zr;
        let zi2 = zi * zi;
        let magnitude_sq = zr2 + zi2;

        // Check escape condition for each pixel
        let mag_arr = magnitude_sq.as_array();
        for i in 0..4 {
            if active_mask[i] && mag_arr[i] > 4.0 {
                iterations[i] = iter;
                active_mask[i] = false;
            }
        }

        // z = z² + c
        let new_zr = zr2 - zi2 + cx_vec;
        let new_zi = two * zr * zi + cy_vec;

        zr = new_zr;
        zi = new_zi;
    }

    // Set remaining active pixels to max_iteration
    for i in 0..4 {
        if active_mask[i] {
            iterations[i] = max_iteration;
        }
    }

    iterations
}

/// SIMD Mandelbrot kernel processing 2 f64 pixels simultaneously.
#[inline(always)]
pub fn mandelbrot_simd_f64(cx: &[f64; 2], cy: &[f64; 2], max_iteration: u16) -> [u16; 2] {
    let mut iterations = [0u16; 2];
    let mut active_mask = [true; 2];

    // Early exit checks
    for i in 0..2 {
        let x_offset = cx[i] - 0.25;
        let q = x_offset * x_offset + cy[i] * cy[i];
        if q * (q + x_offset) < 0.25 * cy[i] * cy[i] {
            iterations[i] = max_iteration;
            active_mask[i] = false;
            continue;
        }
        let x_plus = cx[i] + 1.0;
        if x_plus * x_plus + cy[i] * cy[i] < 0.0625 {
            iterations[i] = max_iteration;
            active_mask[i] = false;
        }
    }

    if !active_mask.iter().any(|&b| b) {
        return iterations;
    }

    let cx_vec = f64x2::from(*cx);
    let cy_vec = f64x2::from(*cy);

    let mut zr = f64x2::ZERO;
    let mut zi = f64x2::ZERO;
    let two = f64x2::splat(2.0);

    for iter in 0..max_iteration {
        if !active_mask.iter().any(|&b| b) {
            break;
        }

        let zr2 = zr * zr;
        let zi2 = zi * zi;
        let magnitude_sq = zr2 + zi2;

        let mag_arr = magnitude_sq.as_array();
        for i in 0..2 {
            if active_mask[i] && mag_arr[i] > 4.0 {
                iterations[i] = iter;
                active_mask[i] = false;
            }
        }

        let new_zr = zr2 - zi2 + cx_vec;
        let new_zi = two * zr * zi + cy_vec;

        zr = new_zr;
        zi = new_zi;
    }

    for i in 0..2 {
        if active_mask[i] {
            iterations[i] = max_iteration;
        }
    }

    iterations
}

// ============================================================================
// JULIA SIMD KERNELS
// ============================================================================

/// SIMD Julia kernel processing 4 f32 pixels simultaneously.
#[inline(always)]
pub fn julia_simd_f32(
    zx: &[f32; 4],
    zy: &[f32; 4],
    cx: f32,
    cy: f32,
    max_iteration: u16,
) -> [u16; 4] {
    let mut x = f32x4::from(*zx);
    let mut y = f32x4::from(*zy);
    let cx_vec = f32x4::splat(cx);
    let cy_vec = f32x4::splat(cy);

    let mut iterations = [0u16; 4];
    let mut active_mask = [true; 4];

    let two = f32x4::splat(2.0);

    for iter in 0..max_iteration {
        if !active_mask.iter().any(|&b| b) {
            break;
        }

        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let mag_arr = magnitude_sq.as_array();
        for i in 0..4 {
            if active_mask[i] && mag_arr[i] > 4.0 {
                iterations[i] = iter;
                active_mask[i] = false;
            }
        }

        // z = z² + c
        let new_y = two * x * y + cy_vec;
        x = x2 - y2 + cx_vec;
        y = new_y;
    }

    for i in 0..4 {
        if active_mask[i] {
            iterations[i] = max_iteration;
        }
    }

    iterations
}

/// SIMD Julia kernel processing 2 f64 pixels simultaneously.
#[inline(always)]
pub fn julia_simd_f64(
    zx: &[f64; 2],
    zy: &[f64; 2],
    cx: f64,
    cy: f64,
    max_iteration: u16,
) -> [u16; 2] {
    let mut x = f64x2::from(*zx);
    let mut y = f64x2::from(*zy);
    let cx_vec = f64x2::splat(cx);
    let cy_vec = f64x2::splat(cy);

    let mut iterations = [0u16; 2];
    let mut active_mask = [true; 2];

    let two = f64x2::splat(2.0);

    for iter in 0..max_iteration {
        if !active_mask.iter().any(|&b| b) {
            break;
        }

        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let mag_arr = magnitude_sq.as_array();
        for i in 0..2 {
            if active_mask[i] && mag_arr[i] > 4.0 {
                iterations[i] = iter;
                active_mask[i] = false;
            }
        }

        let new_y = two * x * y + cy_vec;
        x = x2 - y2 + cx_vec;
        y = new_y;
    }

    for i in 0..2 {
        if active_mask[i] {
            iterations[i] = max_iteration;
        }
    }

    iterations
}

// ============================================================================
// BURNING SHIP SIMD KERNELS
// ============================================================================

/// SIMD Burning Ship kernel processing 4 f32 pixels simultaneously.
#[inline(always)]
pub fn burning_ship_simd_f32(cx: &[f32; 4], cy: &[f32; 4], max_iteration: u16) -> [u16; 4] {
    let cx_vec = f32x4::from(*cx);
    let cy_vec = f32x4::from(*cy);

    let mut x = f32x4::ZERO;
    let mut y = f32x4::ZERO;
    let mut iterations = [0u16; 4];
    let mut active_mask = [true; 4];

    let two = f32x4::splat(2.0);

    for iter in 0..max_iteration {
        if !active_mask.iter().any(|&b| b) {
            break;
        }

        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let mag_arr = magnitude_sq.as_array();
        for i in 0..4 {
            if active_mask[i] && mag_arr[i] > 4.0 {
                iterations[i] = iter;
                active_mask[i] = false;
            }
        }

        // Burning Ship uses abs() values
        let temp = x2 - y2 + cx_vec;
        y = two * x.abs() * y.abs() + cy_vec;
        x = temp;
    }

    for i in 0..4 {
        if active_mask[i] {
            iterations[i] = max_iteration;
        }
    }

    iterations
}

/// SIMD Burning Ship kernel processing 2 f64 pixels simultaneously.
#[inline(always)]
pub fn burning_ship_simd_f64(cx: &[f64; 2], cy: &[f64; 2], max_iteration: u16) -> [u16; 2] {
    let cx_vec = f64x2::from(*cx);
    let cy_vec = f64x2::from(*cy);

    let mut x = f64x2::ZERO;
    let mut y = f64x2::ZERO;
    let mut iterations = [0u16; 2];
    let mut active_mask = [true; 2];

    let two = f64x2::splat(2.0);

    for iter in 0..max_iteration {
        if !active_mask.iter().any(|&b| b) {
            break;
        }

        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let mag_arr = magnitude_sq.as_array();
        for i in 0..2 {
            if active_mask[i] && mag_arr[i] > 4.0 {
                iterations[i] = iter;
                active_mask[i] = false;
            }
        }

        let temp = x2 - y2 + cx_vec;
        y = two * x.abs() * y.abs() + cy_vec;
        x = temp;
    }

    for i in 0..2 {
        if active_mask[i] {
            iterations[i] = max_iteration;
        }
    }

    iterations
}

// ============================================================================
// TRICORN SIMD KERNELS
// ============================================================================

/// SIMD Tricorn kernel processing 4 f32 pixels simultaneously.
#[inline(always)]
pub fn tricorn_simd_f32(cx: &[f32; 4], cy: &[f32; 4], max_iteration: u16) -> [u16; 4] {
    let cx_vec = f32x4::from(*cx);
    let cy_vec = f32x4::from(*cy);

    let mut x = f32x4::ZERO;
    let mut y = f32x4::ZERO;
    let mut iterations = [0u16; 4];
    let mut active_mask = [true; 4];

    let neg_two = f32x4::splat(-2.0);

    for iter in 0..max_iteration {
        if !active_mask.iter().any(|&b| b) {
            break;
        }

        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let mag_arr = magnitude_sq.as_array();
        for i in 0..4 {
            if active_mask[i] && mag_arr[i] > 4.0 {
                iterations[i] = iter;
                active_mask[i] = false;
            }
        }

        // Tricorn uses conjugate
        let temp = x2 - y2 + cx_vec;
        y = neg_two * x * y + cy_vec;
        x = temp;
    }

    for i in 0..4 {
        if active_mask[i] {
            iterations[i] = max_iteration;
        }
    }

    iterations
}

/// SIMD Tricorn kernel processing 2 f64 pixels simultaneously.
#[inline(always)]
pub fn tricorn_simd_f64(cx: &[f64; 2], cy: &[f64; 2], max_iteration: u16) -> [u16; 2] {
    let cx_vec = f64x2::from(*cx);
    let cy_vec = f64x2::from(*cy);

    let mut x = f64x2::ZERO;
    let mut y = f64x2::ZERO;
    let mut iterations = [0u16; 2];
    let mut active_mask = [true; 2];

    let neg_two = f64x2::splat(-2.0);

    for iter in 0..max_iteration {
        if !active_mask.iter().any(|&b| b) {
            break;
        }

        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let mag_arr = magnitude_sq.as_array();
        for i in 0..2 {
            if active_mask[i] && mag_arr[i] > 4.0 {
                iterations[i] = iter;
                active_mask[i] = false;
            }
        }

        let temp = x2 - y2 + cx_vec;
        y = neg_two * x * y + cy_vec;
        x = temp;
    }

    for i in 0..2 {
        if active_mask[i] {
            iterations[i] = max_iteration;
        }
    }

    iterations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandelbrot_simd_f32() {
        let cx = [0.0, -0.5, -1.0, 0.25];
        let cy = [0.0, 0.0, 0.0, 0.0];
        let iterations = mandelbrot_simd_f32(&cx, &cy, 100);

        // All should produce valid iteration counts
        for &iter in &iterations {
            assert!(iter <= 100);
        }
    }

    #[test]
    fn test_mandelbrot_simd_f64() {
        let cx = [0.0, -0.5];
        let cy = [0.0, 0.0];
        let iterations = mandelbrot_simd_f64(&cx, &cy, 100);

        for &iter in &iterations {
            assert!(iter <= 100);
        }
    }

    #[test]
    fn test_julia_simd_f32() {
        let zx = [0.0, 0.1, 0.2, 0.3];
        let zy = [0.0, 0.1, 0.2, 0.3];
        let iterations = julia_simd_f32(&zx, &zy, 0.355, 0.355, 100);

        for &iter in &iterations {
            assert!(iter <= 100);
        }
    }

    #[test]
    fn test_burning_ship_simd_f32() {
        let cx = [0.0, -0.5, -1.0, -1.5];
        let cy = [0.0, -0.5, -0.5, -0.5];
        let iterations = burning_ship_simd_f32(&cx, &cy, 100);

        for &iter in &iterations {
            assert!(iter <= 100);
        }
    }

    #[test]
    fn test_tricorn_simd_f32() {
        let cx = [0.0, -0.5, -1.0, 0.25];
        let cy = [0.0, 0.0, 0.0, 0.0];
        let iterations = tricorn_simd_f32(&cx, &cy, 100);

        for &iter in &iterations {
            assert!(iter <= 100);
        }
    }
}
