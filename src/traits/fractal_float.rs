/// A trait for representing floating-point numbers in a fractal context.
pub trait FractalFloat: Clone + PartialOrd {
    fn zero() -> Self; // Represents the zero value.
    fn two() -> Self; // Represents the two value.
    fn four() -> Self; // Represents the four value.
    fn abs(&self) -> Self; // Returns the absolute value.
    fn from_f64(val: f64) -> Self; // Converts a f64 to the implementing type.
    fn to_f64(&self) -> f64; // Converts the implementing type to f64.
    fn add(&self, other: &Self) -> Self; // Adds two values.
    fn sub(&self, other: &Self) -> Self; // Subtracts two values.
    fn mul(&self, other: &Self) -> Self; // Multiplies two values.
}
