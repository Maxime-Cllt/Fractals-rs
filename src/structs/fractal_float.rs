use crate::traits::fractal_float::FractalFloat;

/// Implementing the FractalFloat trait for f32 (Fast mode)
impl FractalFloat for f32 {
    #[inline]
    fn zero() -> Self {
        0.0
    }

    #[inline]
    fn two() -> Self {
        2.0
    }

    #[inline]
    fn four() -> Self {
        4.0
    }

    #[inline]
    fn abs(&self) -> Self {
        (*self).abs()
    }

    #[inline]
    fn from_f64(val: f64) -> Self {
        val as f32
    }

    #[inline]
    fn to_f64(&self) -> f64 {
        *self as f64
    }

    #[inline]
    fn add(&self, other: &Self) -> Self {
        self + other
    }

    #[inline]
    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    #[inline]
    fn mul(&self, other: &Self) -> Self {
        self * other
    }
}

/// Implementation of the `FractalFloat` trait for `f64` (High Precision Mode).
impl FractalFloat for f64 {
    #[inline]
    fn zero() -> Self {
        0.0
    }

    #[inline]
    fn two() -> Self {
        2.0
    }

    #[inline]
    fn four() -> Self {
        4.0
    }

    #[inline]
    fn abs(&self) -> Self {
        (*self).abs()
    }

    #[inline]
    fn from_f64(val: f64) -> Self {
        val
    }

    #[inline]
    fn to_f64(&self) -> f64 {
        *self
    }

    #[inline]
    fn add(&self, other: &Self) -> Self {
        self + other
    }

    #[inline]
    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    #[inline]
    fn mul(&self, other: &Self) -> Self {
        self * other
    }
}
