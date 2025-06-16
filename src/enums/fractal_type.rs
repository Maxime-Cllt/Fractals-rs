use crate::enums::precision_mode::PrecisionMode;
use crate::structs::point::Point;
use crate::traits::fractal_float::FractalFloat;

#[derive(Clone, Copy, PartialEq)]
pub enum FractalType {
    Mandelbrot,
    Julia,
    BurningShip,
    Tricorn,
}

impl FractalType {
    /// Returns the number of iterations with specified precision mode
    pub fn iterations(&self, cx: f64, cy: f64, max_iteration: u16, julia_c: &Point, precision: PrecisionMode) -> u16 {
        match precision {
            PrecisionMode::Fast => {
                let cx_f32 = cx as f32;
                let cy_f32 = cy as f32;
                match self {
                    FractalType::Mandelbrot => Self::mandelbrot_iterations_generic(cx_f32, cy_f32, max_iteration),
                    FractalType::Julia => Self::julia_iterations_generic(cx_f32, cy_f32, max_iteration, julia_c),
                    FractalType::BurningShip => Self::burning_ship_iterations_generic(cx_f32, cy_f32, max_iteration),
                    FractalType::Tricorn => Self::tricorn_iterations_generic(cx_f32, cy_f32, max_iteration),
                }
            }
            PrecisionMode::High => {
                match self {
                    FractalType::Mandelbrot => Self::mandelbrot_iterations_generic(cx, cy, max_iteration),
                    FractalType::Julia => Self::julia_iterations_generic(cx, cy, max_iteration, julia_c),
                    FractalType::BurningShip => Self::burning_ship_iterations_generic(cx, cy, max_iteration),
                    FractalType::Tricorn => Self::tricorn_iterations_generic(cx, cy, max_iteration),
                }
            }
        }
    }

    #[inline]
    fn mandelbrot_iterations_generic<T: FractalFloat>(cx: T, cy: T, max_iteration: u16) -> u16 {
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
            let new_zr = zr2.sub(&zi2).add(&cx);
            zi = T::two().mul(&zr).mul(&zi).add(&cy);
            zr = new_zr;

            iterations += 1;
        }

        iterations
    }

    #[inline]
    fn julia_iterations_generic<T: FractalFloat>(zx: T, zy: T, max_iteration: u16, c: &Point) -> u16 {
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

    #[inline]
    fn burning_ship_iterations_generic<T: FractalFloat>(cx: T, cy: T, max_iteration: u16) -> u16 {
        let mut x = T::zero();
        let mut y = T::zero();
        let mut iterations = 0u16;

        while iterations < max_iteration {
            let x2 = x.mul(&x);
            let y2 = y.mul(&y);

            if x2.add(&y2) > T::four() {
                break;
            }

            let temp = x2.sub(&y2).add(&cx);
            y = T::two().mul(&x.abs()).mul(&y.abs()).add(&cy);
            x = temp;
            iterations += 1;
        }
        iterations
    }

    #[inline]
    fn tricorn_iterations_generic<T: FractalFloat>(cx: T, cy: T, max_iteration: u16) -> u16 {
        let mut x = T::zero();
        let mut y = T::zero();
        let mut iterations = 0u16;

        while iterations < max_iteration {
            let x2 = x.mul(&x);
            let y2 = y.mul(&y);

            if x2.add(&y2) > T::four() {
                break;
            }

            let temp = x2.sub(&y2).add(&cx);
            y = T::from_f64(-2.0).mul(&x).mul(&y).add(&cy);
            x = temp;
            iterations += 1;
        }
        iterations
    }

    pub const fn name(&self) -> &'static str {
        match self {
            FractalType::Mandelbrot => "Mandelbrot Set",
            FractalType::Julia => "Julia Set",
            FractalType::BurningShip => "Burning Ship",
            FractalType::Tricorn => "Tricorn",
        }
    }

    pub const fn default_center(&self) -> Point {
        match self {
            FractalType::Mandelbrot => Point::new(-0.5, 0.0),
            FractalType::Julia => Point::new(0.0, 0.0),
            FractalType::BurningShip => Point::new(-0.5, -0.5),
            FractalType::Tricorn => Point::new(0.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandelbrot_iterations() {
        let iterations = FractalType::Mandelbrot.iterations(0.0, 0.0, 1000, &Point::new(0.0, 0.0), PrecisionMode::Fast);
        assert!(iterations > 0);
    }

    #[test]
    fn test_julia_iterations() {
        let iterations = FractalType::Julia.iterations(0.0, 0.0, 1000, &Point::new(0.355, 0.355), PrecisionMode::Fast);
        assert!(iterations > 0);
    }

    #[test]
    fn test_burning_ship_iterations() {
        let iterations = FractalType::BurningShip.iterations(0.0, 0.0, 1000, &Point::new(0.0, 0.0), PrecisionMode::Fast);
        assert!(iterations > 0);
    }

    #[test]
    fn test_tricorn_iterations() {
        let iterations = FractalType::Tricorn.iterations(0.0, 0.0, 1000, &Point::new(0.0, 0.0), PrecisionMode::Fast);
        assert!(iterations > 0);
    }
}
