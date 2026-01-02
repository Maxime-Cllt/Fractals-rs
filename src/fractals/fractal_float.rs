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

/// Implementing the `FractalFloat` trait for f32 (Fast mode)
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
        val as Self
    }

    #[inline]
    fn to_f64(&self) -> f64 {
        f64::from(*self)
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

#[cfg(feature = "f128")]
/// Implementation of the `FractalFloat` trait for `rust_decimal::Decimal` (Ultra High Precision Mode).
/// This enables 128-bit decimal precision for extreme zoom levels.
impl FractalFloat for rust_decimal::Decimal {
    #[inline]
    fn zero() -> Self {
        rust_decimal::Decimal::ZERO
    }

    #[inline]
    fn two() -> Self {
        rust_decimal::Decimal::TWO
    }

    #[inline]
    fn four() -> Self {
        rust_decimal_macros::dec!(4)
    }

    #[inline]
    fn abs(&self) -> Self {
        (*self).abs()
    }

    #[inline]
    fn from_f64(val: f64) -> Self {
        rust_decimal::Decimal::from_f64_retain(val).unwrap_or(rust_decimal::Decimal::ZERO)
    }

    #[inline]
    fn to_f64(&self) -> f64 {
        // Call the to_f64 method from ToPrimitive trait via f64::try_from
        f64::try_from(*self).unwrap_or(0.0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractal_float_f32() {
        let a: f32 = 1.5;
        let b: f32 = 2.5;

        // Test arithmetic operations
        assert_eq!(a.add(&b), 4.0_f32);
        assert_eq!(a.sub(&b), -1.0_f32);
        assert_eq!(a.mul(&b), 3.75_f32);

        // Test abs
        assert_eq!(a.abs(), 1.5_f32);

        // Test conversion functions
        assert_eq!(<f32 as FractalFloat>::from_f64(3.0), 3.0_f32);
        assert_eq!(a.to_f64(), 1.5_f64);

        // Test constant functions with explicit type specification
        assert_eq!(<f32 as FractalFloat>::zero(), 0.0_f32);
        assert_eq!(<f32 as FractalFloat>::two(), 2.0_f32);
        assert_eq!(<f32 as FractalFloat>::four(), 4.0_f32);
    }

    #[test]
    fn test_fractal_float_f64() {
        let a: f64 = 1.5;
        let b: f64 = 2.5;

        // Test arithmetic operations
        assert_eq!(a.add(&b), 4.0_f64);
        assert_eq!(a.sub(&b), -1.0_f64);
        assert_eq!(a.mul(&b), 3.75_f64);

        // Test abs
        assert_eq!(a.abs(), 1.5_f64);

        // Test conversion functions
        assert_eq!(<f64 as FractalFloat>::from_f64(3.0), 3.0_f64);
        assert_eq!(a.to_f64(), 1.5_f64);

        // Test constant functions with explicit type specification
        assert_eq!(<f64 as FractalFloat>::zero(), 0.0_f64);
        assert_eq!(<f64 as FractalFloat>::two(), 2.0_f64);
        assert_eq!(<f64 as FractalFloat>::four(), 4.0_f64);
    }

    #[test]
    fn test_negative_abs() {
        let neg_f32: f32 = -3.5;
        let neg_f64: f64 = -3.5;

        assert_eq!(neg_f32.abs(), 3.5_f32);
        assert_eq!(neg_f64.abs(), 3.5_f64);
    }

    #[test]
    fn test_conversion_precision() {
        // Test f64 to f32 conversion (potential precision loss)
        let large_f64 = 1.23456789012345_f64;
        let converted_f32 = <f32 as FractalFloat>::from_f64(large_f64);

        // Convert back to f64 to check
        let back_to_f64 = converted_f32.to_f64();

        // Should be approximately equal due to f32 precision limits
        assert!((back_to_f64 - large_f64).abs() < 1e-6);
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero
        let zero_f32 = 0.0_f32;
        let zero_f64 = 0.0_f64;

        assert_eq!(zero_f32.abs(), 0.0_f32);
        assert_eq!(zero_f64.abs(), 0.0_f64);

        // Test arithmetic with zero
        let val_f32 = 5.0_f32;
        let val_f64 = 5.0_f64;

        assert_eq!(val_f32.add(&zero_f32), 5.0_f32);
        assert_eq!(val_f32.sub(&zero_f32), 5.0_f32);
        assert_eq!(val_f32.mul(&zero_f32), 0.0_f32);

        assert_eq!(val_f64.add(&zero_f64), 5.0_f64);
        assert_eq!(val_f64.sub(&zero_f64), 5.0_f64);
        assert_eq!(val_f64.mul(&zero_f64), 0.0_f64);
    }

    #[cfg(feature = "f128")]
    #[test]
    fn test_fractal_float_decimal() {
        use rust_decimal::Decimal;

        let a = Decimal::from_f64_retain(1.5).unwrap();
        let b = Decimal::from_f64_retain(2.5).unwrap();

        // Test arithmetic operations
        let sum = a.add(&b);
        assert_eq!(sum.to_f64(), 4.0_f64);

        let diff = a.sub(&b);
        assert_eq!(diff.to_f64(), -1.0_f64);

        let prod = a.mul(&b);
        assert_eq!(prod.to_f64(), 3.75_f64);

        // Test abs
        assert_eq!(a.abs().to_f64(), 1.5_f64);

        // Test conversion functions
        let converted = <Decimal as FractalFloat>::from_f64(3.0);
        assert_eq!(converted.to_f64(), 3.0_f64);

        // Test constant functions
        assert_eq!(<Decimal as FractalFloat>::zero().to_f64(), 0.0_f64);
        assert_eq!(<Decimal as FractalFloat>::two().to_f64(), 2.0_f64);
        assert_eq!(<Decimal as FractalFloat>::four().to_f64(), 4.0_f64);
    }
}
