use crate::traits::fractal_float::FractalFloat;

/// Implementing the FractalFloat trait for f32 (Fast mode)
impl FractalFloat for f32 {
    fn zero() -> Self {
        0.0
    }
    fn two() -> Self {
        2.0
    }
    fn four() -> Self {
        4.0
    }
    fn abs(&self) -> Self {
        (*self).abs()
    }
    fn from_f64(val: f64) -> Self {
        val as f32
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn add(&self, other: &Self) -> Self {
        self + other
    }
    fn sub(&self, other: &Self) -> Self {
        self - other
    }
    fn mul(&self, other: &Self) -> Self {
        self * other
    }
}

/// Implementation of the `FractalFloat` trait for `f64` (High Precision Mode).
impl FractalFloat for f64 {
    fn zero() -> Self {
        0.0
    }
    fn two() -> Self {
        2.0
    }
    fn four() -> Self {
        4.0
    }
    fn abs(&self) -> Self {
        (*self).abs()
    }
    fn from_f64(val: f64) -> Self {
        val
    }
    fn to_f64(&self) -> f64 {
        *self
    }
    fn add(&self, other: &Self) -> Self {
        self + other
    }
    fn sub(&self, other: &Self) -> Self {
        self - other
    }
    fn mul(&self, other: &Self) -> Self {
        self * other
    }
}
