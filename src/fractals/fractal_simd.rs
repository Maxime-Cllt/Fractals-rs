/// SIMD-accelerated fractal computation kernels.
/// Uses the `wide` crate for portable SIMD operations across platforms.
///
/// Key optimizations:
/// - Process 4 pixels simultaneously with f32x4 (NEON / SSE)
/// - Process 2 pixels simultaneously with f64x2
/// - FMA (fused multiply-add) for the `new_zi` update in every kernel
/// - Vectorized Mandelbrot cardioid + period-2 bulb early-out (single SIMD pass)
/// - move_mask() for O(1) escape detection (1 SIMD instruction vs as_array + 4 scalar cmps)
/// - Bitmask-based active tracking (no bool array on stack)
use wide::{CmpGt, CmpLt, f32x4, f64x2};

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
    let cx_vec = f32x4::from(*cx);
    let cy_vec = f32x4::from(*cy);
    let cy_sq = cy_vec * cy_vec;
    let quarter = f32x4::splat(0.25);

    // Vectorized cardioid + period-2 bulb early-out (single SIMD pass for all 4 lanes)
    let x_offset = cx_vec - quarter;
    let q = x_offset.mul_add(x_offset, cy_sq);
    let cardioid_in = (q * (q + x_offset)).simd_lt(quarter * cy_sq);
    let x_plus = cx_vec + f32x4::splat(1.0);
    let bulb_in = x_plus.mul_add(x_plus, cy_sq).simd_lt(f32x4::splat(0.0625));
    let in_set_bits = (cardioid_in | bulb_in).to_bitmask() as u32 & 0xF;

    let mut iterations = [0u16; 4];
    if in_set_bits & 0x1 != 0 { iterations[0] = max_iteration; }
    if in_set_bits & 0x2 != 0 { iterations[1] = max_iteration; }
    if in_set_bits & 0x4 != 0 { iterations[2] = max_iteration; }
    if in_set_bits & 0x8 != 0 { iterations[3] = max_iteration; }

    let mut active_bits = !in_set_bits & 0xF;
    if active_bits == 0 {
        return iterations;
    }

    let mut zr = f32x4::ZERO;
    let mut zi = f32x4::ZERO;
    let two = f32x4::splat(2.0);
    let four = f32x4::splat(4.0);

    for iter in 0..max_iteration {
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        let magnitude_sq = zr2 + zi2;

        // move_mask: 1 SIMD instruction (MOVMSKPS) — bit i = sign bit of lane i
        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0xF;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            // Only runs when a lane escapes — branch well-predicted as not-taken
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            if newly_escaped & 0x4 != 0 { iterations[2] = iter; }
            if newly_escaped & 0x8 != 0 { iterations[3] = iter; }
            active_bits &= !escaped_bits;
            if active_bits == 0 {
                return iterations;
            }
        }

        // z = z² + c (FMA on new_zi saves 1 op vs `two*zr*zi + cy_vec`)
        let new_zr = zr2 - zi2 + cx_vec;
        let new_zi = (zr * two).mul_add(zi, cy_vec);
        zr = new_zr;
        zi = new_zi;
    }

    // Remaining active lanes hit max_iteration (in-set)
    if active_bits & 0x1 != 0 { iterations[0] = max_iteration; }
    if active_bits & 0x2 != 0 { iterations[1] = max_iteration; }
    if active_bits & 0x4 != 0 { iterations[2] = max_iteration; }
    if active_bits & 0x8 != 0 { iterations[3] = max_iteration; }

    iterations
}

/// SIMD Mandelbrot kernel processing 2 f64 pixels simultaneously.
#[inline(always)]
pub fn mandelbrot_simd_f64(cx: &[f64; 2], cy: &[f64; 2], max_iteration: u16) -> [u16; 2] {
    // Cardioid + period-2 bulb early-out: keep scalar for f64x2 since the
    // SIMD setup overhead (splats, to_bitmask) outweighs the gain on 2 lanes.
    let mut iterations = [0u16; 2];
    let mut active_bits: u32 = 0b11;
    for i in 0..2 {
        let x_offset = cx[i] - 0.25;
        let q = x_offset.mul_add(x_offset, cy[i] * cy[i]);
        if q * (q + x_offset) < 0.25 * cy[i] * cy[i] {
            iterations[i] = max_iteration;
            active_bits &= !(1u32 << i);
            continue;
        }
        let x_plus = cx[i] + 1.0;
        if x_plus.mul_add(x_plus, cy[i] * cy[i]) < 0.0625 {
            iterations[i] = max_iteration;
            active_bits &= !(1u32 << i);
        }
    }

    if active_bits == 0 {
        return iterations;
    }

    let cx_vec = f64x2::from(*cx);
    let cy_vec = f64x2::from(*cy);
    let mut zr = f64x2::ZERO;
    let mut zi = f64x2::ZERO;
    let two = f64x2::splat(2.0);
    let four = f64x2::splat(4.0);

    for iter in 0..max_iteration {
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        let magnitude_sq = zr2 + zi2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0x3;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            active_bits &= !escaped_bits;
            if active_bits == 0 {
                return iterations;
            }
        }

        // FMA on new_zi saves 1 op vs `two * zr * zi + cy_vec`
        // FMA on new_zi saves 1 op vs `two * zr * zi + cy_vec`
        let new_zr = zr2 - zi2 + cx_vec;
        let new_zi = (zr * two).mul_add(zi, cy_vec);
        zr = new_zr;
        zi = new_zi;
    }

    if active_bits & 0x1 != 0 { iterations[0] = max_iteration; }
    if active_bits & 0x2 != 0 { iterations[1] = max_iteration; }

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
    let mut active_bits: u32 = 0b1111;
    let two = f32x4::splat(2.0);
    let four = f32x4::splat(4.0);

    for iter in 0..max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0xF;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            if newly_escaped & 0x4 != 0 { iterations[2] = iter; }
            if newly_escaped & 0x8 != 0 { iterations[3] = iter; }
            active_bits &= !escaped_bits;
            if active_bits == 0 {
                return iterations;
            }
        }

        // z = z² + c with FMA on new_y
        let new_y = (x * two).mul_add(y, cy_vec);
        x = x2 - y2 + cx_vec;
        y = new_y;
    }

    if active_bits & 0x1 != 0 { iterations[0] = max_iteration; }
    if active_bits & 0x2 != 0 { iterations[1] = max_iteration; }
    if active_bits & 0x4 != 0 { iterations[2] = max_iteration; }
    if active_bits & 0x8 != 0 { iterations[3] = max_iteration; }

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
    let mut active_bits: u32 = 0b11;
    let two = f64x2::splat(2.0);
    let four = f64x2::splat(4.0);

    for iter in 0..max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0x3;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            active_bits &= !escaped_bits;
            if active_bits == 0 {
                return iterations;
            }
        }

        let new_y = (x * two).mul_add(y, cy_vec);
        x = (x2 + cx_vec) - y2;
        y = new_y;
    }

    if active_bits & 0x1 != 0 { iterations[0] = max_iteration; }
    if active_bits & 0x2 != 0 { iterations[1] = max_iteration; }

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
    let mut active_bits: u32 = 0b1111;
    let two = f32x4::splat(2.0);
    let four = f32x4::splat(4.0);

    for iter in 0..max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0xF;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            if newly_escaped & 0x4 != 0 { iterations[2] = iter; }
            if newly_escaped & 0x8 != 0 { iterations[3] = iter; }
            active_bits &= !escaped_bits;
            if active_bits == 0 {
                return iterations;
            }
        }

        // Burning Ship: y = 2*|x|*|y| + cy with FMA
        let temp = x2 - y2 + cx_vec;
        y = (x.abs() * two).mul_add(y.abs(), cy_vec);
        x = temp;
    }

    if active_bits & 0x1 != 0 { iterations[0] = max_iteration; }
    if active_bits & 0x2 != 0 { iterations[1] = max_iteration; }
    if active_bits & 0x4 != 0 { iterations[2] = max_iteration; }
    if active_bits & 0x8 != 0 { iterations[3] = max_iteration; }

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
    let mut active_bits: u32 = 0b11;
    let two = f64x2::splat(2.0);
    let four = f64x2::splat(4.0);

    for iter in 0..max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0x3;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            active_bits &= !escaped_bits;
            if active_bits == 0 {
                return iterations;
            }
        }

        let temp = x2 - y2 + cx_vec;
        y = (x.abs() * two).mul_add(y.abs(), cy_vec);
        x = temp;
    }

    if active_bits & 0x1 != 0 { iterations[0] = max_iteration; }
    if active_bits & 0x2 != 0 { iterations[1] = max_iteration; }

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
    let mut active_bits: u32 = 0b1111;
    let neg_two = f32x4::splat(-2.0);
    let four = f32x4::splat(4.0);

    for iter in 0..max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0xF;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            if newly_escaped & 0x4 != 0 { iterations[2] = iter; }
            if newly_escaped & 0x8 != 0 { iterations[3] = iter; }
            active_bits &= !escaped_bits;
            if active_bits == 0 {
                return iterations;
            }
        }

        // Tricorn uses conjugate: y = -2*x*y + cy with FMA
        let temp = x2 - y2 + cx_vec;
        y = (x * neg_two).mul_add(y, cy_vec);
        x = temp;
    }

    if active_bits & 0x1 != 0 { iterations[0] = max_iteration; }
    if active_bits & 0x2 != 0 { iterations[1] = max_iteration; }
    if active_bits & 0x4 != 0 { iterations[2] = max_iteration; }
    if active_bits & 0x8 != 0 { iterations[3] = max_iteration; }

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
    let mut active_bits: u32 = 0b11;
    let neg_two = f64x2::splat(-2.0);
    let four = f64x2::splat(4.0);

    for iter in 0..max_iteration {
        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0x3;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            active_bits &= !escaped_bits;
            if active_bits == 0 {
                return iterations;
            }
        }

        let temp = x2 - y2 + cx_vec;
        y = (x * neg_two).mul_add(y, cy_vec);
        x = temp;
    }

    if active_bits & 0x1 != 0 { iterations[0] = max_iteration; }
    if active_bits & 0x2 != 0 { iterations[1] = max_iteration; }

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

    /// Verify the SIMD kernels match the scalar kernels on a grid of points.
    /// Catches regressions in the FMA-restructured inner loop or the
    /// vectorized cardioid check.
    #[test]
    fn test_simd_matches_scalar() {
        use crate::fractals::fractal_kernels;
        let mi = 200u16;
        // Sample a grid that spans both escape and in-set regions.
        for yi in -10..=10 {
            let cy_f32 = yi as f32 * 0.1;
            for xi in -20..=10 {
                let cx_f32 = xi as f32 * 0.1;
                let scalar = fractal_kernels::mandelbrot_iterations_f32(cx_f32, cy_f32, mi);
                let simd = mandelbrot_simd_f32(
                    &[cx_f32; 4],
                    &[cy_f32; 4],
                    mi,
                );
                // SIMD output may differ by ±1 due to op-ordering differences
                // (FMA vs separate mul+add changes intermediate rounding).
                for &s in &simd {
                    let diff = (scalar as i32 - s as i32).abs();
                    assert!(
                        diff <= 1,
                        "Mandelbrot SIMD diverged at ({cx_f32}, {cy_f32}): scalar={scalar} simd={s}"
                    );
                }
            }
        }
    }
}
