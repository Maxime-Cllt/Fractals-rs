/// A trait for representing floating-point numbers in a fractal context.
pub trait FractalFloat: Clone + PartialOrd {
    fn zero() -> Self;
    fn two() -> Self;
    fn four() -> Self;
    fn abs(&self) -> Self;
    fn from_f64(val: f64) -> Self;
    fn to_f64(&self) -> f64;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}
