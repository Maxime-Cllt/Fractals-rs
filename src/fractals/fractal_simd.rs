/// SIMD-accelerated fractal computation kernels.
/// Uses the `wide` crate for portable SIMD operations across platforms.
///
/// Key optimizations:
/// - Process 4 pixels simultaneously with f32x4 (SSE/AVX)
/// - Process 2 pixels simultaneously with f64x2
/// - Vectorized escape-time algorithm
/// - move_mask() for O(1) escape detection (1 SIMD instruction vs as_array + 4 scalar cmps)
/// - Bitmask-based active tracking (no bool array on stack)
use wide::{CmpGt, f32x4, f64x2};

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
    // Bitmask: bit i = 1 means lane i is still active
    let mut active_bits: u32 = 0b1111;

    // Early exit: cardioid and period-2 bulb checks
    for i in 0..4 {
        let x_offset = cx[i] - 0.25;
        let q = x_offset * x_offset + cy[i] * cy[i];
        if q * (q + x_offset) < 0.25 * cy[i] * cy[i] {
            iterations[i] = max_iteration;
            active_bits &= !(1u32 << i);
            continue;
        }
        let x_plus = cx[i] + 1.0;
        if x_plus * x_plus + cy[i] * cy[i] < 0.0625 {
            iterations[i] = max_iteration;
            active_bits &= !(1u32 << i);
        }
    }

    if active_bits == 0 {
        return iterations;
    }

    let cx_vec = f32x4::from(*cx);
    let cy_vec = f32x4::from(*cy);
    let mut zr = f32x4::ZERO;
    let mut zi = f32x4::ZERO;
    let two = f32x4::splat(2.0);
    let four = f32x4::splat(4.0);

    for iter in 0..max_iteration {
        if active_bits == 0 {
            break;
        }

        let zr2 = zr * zr;
        let zi2 = zi * zi;
        let magnitude_sq = zr2 + zi2;

        // move_mask: 1 SIMD instruction (MOVMSKPS) — bit i = sign bit of lane i
        // cmp_gt sets all bits for true lanes, so sign bit is 1 when escaped
        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0xF;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            // Only runs when a lane escapes — branch well-predicted as not-taken
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            if newly_escaped & 0x4 != 0 { iterations[2] = iter; }
            if newly_escaped & 0x8 != 0 { iterations[3] = iter; }
            active_bits &= !escaped_bits;
        }

        // z = z² + c (all lanes — dead lanes are harmless)
        let new_zr = zr2 - zi2 + cx_vec;
        let new_zi = two * zr * zi + cy_vec;
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
    let mut iterations = [0u16; 2];
    let mut active_bits: u32 = 0b11;

    // Early exit checks
    for i in 0..2 {
        let x_offset = cx[i] - 0.25;
        let q = x_offset * x_offset + cy[i] * cy[i];
        if q * (q + x_offset) < 0.25 * cy[i] * cy[i] {
            iterations[i] = max_iteration;
            active_bits &= !(1u32 << i);
            continue;
        }
        let x_plus = cx[i] + 1.0;
        if x_plus * x_plus + cy[i] * cy[i] < 0.0625 {
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
        if active_bits == 0 {
            break;
        }

        let zr2 = zr * zr;
        let zi2 = zi * zi;
        let magnitude_sq = zr2 + zi2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0x3;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            active_bits &= !escaped_bits;
        }

        let new_zr = zr2 - zi2 + cx_vec;
        let new_zi = two * zr * zi + cy_vec;
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
        if active_bits == 0 {
            break;
        }

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
        }

        // z = z² + c
        let new_y = two * x * y + cy_vec;
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
        if active_bits == 0 {
            break;
        }

        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0x3;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            active_bits &= !escaped_bits;
        }

        let new_y = two * x * y + cy_vec;
        x = x2 - y2 + cx_vec;
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
        if active_bits == 0 {
            break;
        }

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
        }

        // Burning Ship uses abs() values
        let temp = x2 - y2 + cx_vec;
        y = two * x.abs() * y.abs() + cy_vec;
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
        if active_bits == 0 {
            break;
        }

        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0x3;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            active_bits &= !escaped_bits;
        }

        let temp = x2 - y2 + cx_vec;
        y = two * x.abs() * y.abs() + cy_vec;
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
        if active_bits == 0 {
            break;
        }

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
        }

        // Tricorn uses conjugate
        let temp = x2 - y2 + cx_vec;
        y = neg_two * x * y + cy_vec;
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
        if active_bits == 0 {
            break;
        }

        let x2 = x * x;
        let y2 = y * y;
        let magnitude_sq = x2 + y2;

        let escaped_bits = magnitude_sq.simd_gt(four).to_bitmask() as u32 & 0x3;
        let newly_escaped = escaped_bits & active_bits;

        if newly_escaped != 0 {
            if newly_escaped & 0x1 != 0 { iterations[0] = iter; }
            if newly_escaped & 0x2 != 0 { iterations[1] = iter; }
            active_bits &= !escaped_bits;
        }

        let temp = x2 - y2 + cx_vec;
        y = neg_two * x * y + cy_vec;
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
}
