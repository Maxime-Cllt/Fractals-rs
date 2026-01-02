use crate::utils::precision_mode::PrecisionMode;
use crate::utils::point::Point;
use crate::fractals::fractal_float::FractalFloat;

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
    /// Returns the number of iterations with specified precision mode
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
                        Self::mandelbrot_iterations_generic(&cx_f32, &cy_f32, max_iteration)
                    }
                    Self::Julia => {
                        Self::julia_iterations_generic(cx_f32, cy_f32, max_iteration, julia_c)
                    }
                    Self::BurningShip => {
                        Self::burning_ship_iterations_generic(&cx_f32, &cy_f32, max_iteration)
                    }
                    Self::Tricorn => {
                        Self::tricorn_iterations_generic(&cx_f32, &cy_f32, max_iteration)
                    }
                }
            }
            PrecisionMode::High => match self {
                Self::Mandelbrot => Self::mandelbrot_iterations_generic(&cx, &cy, max_iteration),
                Self::Julia => Self::julia_iterations_generic(cx, cy, max_iteration, julia_c),
                Self::BurningShip => Self::burning_ship_iterations_generic(&cx, &cy, max_iteration),
                Self::Tricorn => Self::tricorn_iterations_generic(&cx, &cy, max_iteration),
            },
        }
    }

    /// Returns the number of iterations for the Mandelbrot fractal
    #[inline]
    fn mandelbrot_iterations_generic<T: FractalFloat>(cx: &T, cy: &T, max_iteration: u16) -> u16 {
        let mut zr = T::zero();
        let mut zi = T::zero();
        let mut iterations = 0u16;

        while iterations < max_iteration {
            let zr2 = zr.mul(&zr);
            let zi2 = zi.mul(&zi);

            if zr2.add(&zi2) > T::four() {
                break;
            }

            // z = z² + c
            let new_zr = zr2.sub(&zi2).add(cx);
            zi = T::two().mul(&zr).mul(&zi).add(cy);
            zr = new_zr;

            iterations += 1;
        }

        iterations
    }

    /// Returns the number of iterations for the Julia fractal
    #[inline]
    fn julia_iterations_generic<T: FractalFloat>(
        zx: T,
        zy: T,
        max_iteration: u16,
        c: &Point,
    ) -> u16 {
        let mut x = zx;
        let mut y = zy;
        let mut iterations = 0u16;
        let cx = T::from_f64(c.x);
        let cy = T::from_f64(c.y);

        while iterations < max_iteration {
            let x2 = x.mul(&x);
            let y2 = y.mul(&y);

            if x2.add(&y2) > T::four() {
                break;
            }

            let new_y = T::two().mul(&x).mul(&y).add(&cy);
            x = x2.sub(&y2).add(&cx);
            y = new_y;

            iterations += 1;
        }
        iterations
    }

    /// Returns the number of iterations for the Burning Ship fractal
    #[inline]
    fn burning_ship_iterations_generic<T: FractalFloat>(cx: &T, cy: &T, max_iteration: u16) -> u16 {
        let mut x = T::zero();
        let mut y = T::zero();
        let mut iterations = 0u16;

        while iterations < max_iteration {
            let x2 = x.mul(&x);
            let y2 = y.mul(&y);

            if x2.add(&y2) > T::four() {
                break;
            }

            let temp = x2.sub(&y2).add(cx);
            y = T::two().mul(&x.abs()).mul(&y.abs()).add(cy);
            x = temp;
            iterations += 1;
        }
        iterations
    }

    /// Returns the number of iterations for the Tricorn fractal
    #[inline]
    fn tricorn_iterations_generic<T: FractalFloat>(cx: &T, cy: &T, max_iteration: u16) -> u16 {
        let mut x = T::zero();
        let mut y = T::zero();
        let mut iterations = 0u16;

        while iterations < max_iteration {
            let x2 = x.mul(&x);
            let y2 = y.mul(&y);

            if x2.add(&y2) > T::four() {
                break;
            }

            let temp = x2.sub(&y2).add(cx);
            y = T::from_f64(-2.0).mul(&x).mul(&y).add(cy);
            x = temp;
            iterations += 1;
        }
        iterations
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
