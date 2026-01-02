use crate::utils::precision_mode::PrecisionMode;
use crate::utils::point::Point;
use crate::fractals::fractal_kernels;

/// Represents the type of fractal to be generated.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FractalType {
    Mandelbrot,
    Julia,
    BurningShip,
    Tricorn,
}

impl FractalType {
    /// Returns the number of iterations with specified precision mode.
    /// Now using optimized direct kernel implementations for maximum performance.
    #[inline]
    pub fn iterations(
        &self,
        cx: f64,
        cy: f64,
        max_iteration: u16,
        julia_c: &Point,
        precision: PrecisionMode,
    ) -> u16 {
        match precision {
            PrecisionMode::Fast => {
                let cx_f32 = cx as f32;
                let cy_f32 = cy as f32;
                match self {
                    Self::Mandelbrot => {
                        fractal_kernels::mandelbrot_iterations_f32(cx_f32, cy_f32, max_iteration)
                    }
                    Self::Julia => {
                        fractal_kernels::julia_iterations_f32(cx_f32, cy_f32, max_iteration, julia_c)
                    }
                    Self::BurningShip => {
                        fractal_kernels::burning_ship_iterations_f32(cx_f32, cy_f32, max_iteration)
                    }
                    Self::Tricorn => {
                        fractal_kernels::tricorn_iterations_f32(cx_f32, cy_f32, max_iteration)
                    }
                }
            }
            PrecisionMode::High => match self {
                Self::Mandelbrot => {
                    fractal_kernels::mandelbrot_iterations_f64(cx, cy, max_iteration)
                }
                Self::Julia => {
                    fractal_kernels::julia_iterations_f64(cx, cy, max_iteration, julia_c)
                }
                Self::BurningShip => {
                    fractal_kernels::burning_ship_iterations_f64(cx, cy, max_iteration)
                }
                Self::Tricorn => {
                    fractal_kernels::tricorn_iterations_f64(cx, cy, max_iteration)
                }
            },
        }
    }

    /// Returns the name of the fractal type
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Mandelbrot => "Mandelbrot Set",
            Self::Julia => "Julia Set",
            Self::BurningShip => "Burning Ship",
            Self::Tricorn => "Tricorn",
        }
    }

    /// Returns the default center point for the fractal type
    #[inline]
    pub const fn default_center(&self) -> Point {
        match self {
            Self::Mandelbrot => Point::new(-0.5, 0.0),
            Self::Julia | Self::Tricorn => Point::new(0.0, 0.0),
            Self::BurningShip => Point::new(-0.5, -0.5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandelbrot_iterations() {
        let iterations = FractalType::Mandelbrot.iterations(
            0.0,
            0.0,
            1000,
            &Point::new(0.0, 0.0),
            PrecisionMode::Fast,
        );
        assert!(iterations > 0);
    }

    #[test]
    fn test_julia_iterations() {
        let iterations = FractalType::Julia.iterations(
            0.0,
            0.0,
            1000,
            &Point::new(0.355, 0.355),
            PrecisionMode::Fast,
        );
        assert!(iterations > 0);
    }

    #[test]
    fn test_burning_ship_iterations() {
        let iterations = FractalType::BurningShip.iterations(
            0.0,
            0.0,
            1000,
            &Point::new(0.0, 0.0),
            PrecisionMode::Fast,
        );
        assert!(iterations > 0);
    }

    #[test]
    fn test_tricorn_iterations() {
        let iterations = FractalType::Tricorn.iterations(
            0.0,
            0.0,
            1000,
            &Point::new(0.0, 0.0),
            PrecisionMode::Fast,
        );
        assert!(iterations > 0);
    }

    #[test]
    fn test_fractal_type_name() {
        assert_eq!(FractalType::Mandelbrot.name(), "Mandelbrot Set");
        assert_eq!(FractalType::Julia.name(), "Julia Set");
        assert_eq!(FractalType::BurningShip.name(), "Burning Ship");
        assert_eq!(FractalType::Tricorn.name(), "Tricorn");
    }

    #[test]
    fn test_fractal_type_default_center() {
        assert_eq!(
            FractalType::Mandelbrot.default_center(),
            Point::new(-0.5, 0.0)
        );
        assert_eq!(FractalType::Julia.default_center(), Point::new(0.0, 0.0));
        assert_eq!(
            FractalType::BurningShip.default_center(),
            Point::new(-0.5, -0.5)
        );
        assert_eq!(FractalType::Tricorn.default_center(), Point::new(0.0, 0.0));
    }
}
