/// Enum representing different precision modes for numerical computations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PrecisionMode {
    Fast,      // f32 - Standard precision, fastest
    High,      // f64 - Double precision
    #[cfg(feature = "f128")]
    UltraHigh, // f128 - Quadruple precision for extreme zooms (128-bit decimal)
}

#[cfg(test)]
mod tests {
    use super::PrecisionMode;

    #[test]
    fn test_precision_mode_debug() {
        assert_eq!(format!("{:?}", PrecisionMode::Fast), "Fast");
        assert_eq!(format!("{:?}", PrecisionMode::High), "High");
        #[cfg(feature = "f128")]
        assert_eq!(format!("{:?}", PrecisionMode::UltraHigh), "UltraHigh");
    }

    #[test]
    fn test_precision_mode_equality() {
        assert_eq!(PrecisionMode::Fast, PrecisionMode::Fast);
        assert_ne!(PrecisionMode::Fast, PrecisionMode::High);
        assert_ne!(PrecisionMode::High, PrecisionMode::Fast);
        assert_eq!(PrecisionMode::High, PrecisionMode::High);

        #[cfg(feature = "f128")]
        {
            assert_ne!(PrecisionMode::Fast, PrecisionMode::UltraHigh);
            assert_ne!(PrecisionMode::High, PrecisionMode::UltraHigh);
            assert_eq!(PrecisionMode::UltraHigh, PrecisionMode::UltraHigh);
        }
    }
}
