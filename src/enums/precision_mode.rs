/// Enum representing different precision modes for numerical computations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrecisionMode {
    Fast, // f32
    High, // f64
}

#[cfg(test)]
mod tests {
    use super::PrecisionMode;

    #[test]
    fn test_precision_mode_debug() {
        assert_eq!(format!("{:?}", PrecisionMode::Fast), "Fast");
        assert_eq!(format!("{:?}", PrecisionMode::High), "High");
    }

    #[test]
    fn test_precision_mode_equality() {
        assert_eq!(PrecisionMode::Fast, PrecisionMode::Fast);
        assert_ne!(PrecisionMode::Fast, PrecisionMode::High);
        assert_ne!(PrecisionMode::High, PrecisionMode::Fast);
        assert_eq!(PrecisionMode::High, PrecisionMode::High);
    }
}
