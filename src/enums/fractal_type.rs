use crate::structs::point::Point;

#[derive(Clone, Copy, PartialEq)]
pub enum FractalType {
    Mandelbrot,
    Julia,
    BurningShip,
    Tricorn,
}

impl FractalType {
    /// Returns the number of iterations for the given complex coordinate (cx, cy)
    pub fn iterations(&self, cx: f64, cy: f64, max_iteration: u16, julia_c: &Point) -> u16 {
        match self {
            FractalType::Mandelbrot => Self::mandelbrot_iterations(cx, cy, max_iteration),
            FractalType::Julia => Self::julia_iterations(cx, cy, max_iteration, julia_c),
            FractalType::BurningShip => Self::burning_ship_iterations(cx, cy, max_iteration),
            FractalType::Tricorn => Self::tricorn_iterations(cx, cy, max_iteration),
        }
    }

    /// Computes the number of iterations for the Mandelbrot set at the given complex coordinate.
    const fn mandelbrot_iterations(cx: f64, cy: f64, max_iteration: u16) -> u16 {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut iterations: u16 = 0;

        while x * x + y * y <= 4.0 && iterations < max_iteration {
            let temp = x * x - y * y + cx;
            y = 2.0 * x * y + cy;
            x = temp;
            iterations += 1;
        }
        iterations
    }

    /// Julia set iterations
    const fn julia_iterations(zx: f64, zy: f64, max_iteration: u16, c: &Point) -> u16 {
        let mut x: f64 = zx;
        let mut y: f64 = zy;
        let mut iterations: u16 = 0;

        while x * x + y * y <= 4.0 && iterations < max_iteration {
            let temp: f64 = x * x - y * y + c.x;
            y = 2.0 * x * y + c.y;
            x = temp;
            iterations += 1;
        }
        iterations
    }

    /// Burning Ship fractal iterations
    const fn burning_ship_iterations(cx: f64, cy: f64, max_iteration: u16) -> u16 {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut iterations: u16 = 0;

        while x * x + y * y <= 4.0 && iterations < max_iteration {
            let temp = x * x - y * y + cx;
            y = 2.0 * x.abs() * y.abs() + cy;
            x = temp;
            iterations += 1;
        }
        iterations
    }

    /// Tricorn fractal iterations
    const fn tricorn_iterations(cx: f64, cy: f64, max_iteration: u16) -> u16 {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut iterations: u16 = 0;

        while x * x + y * y <= 4.0 && iterations < max_iteration {
            let temp = x * x - y * y + cx;
            y = -2.0 * x * y + cy;
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
        let iterations = FractalType::Mandelbrot.iterations(0.0, 0.0, 1000, &Point::new(0.0, 0.0));
        assert!(iterations > 0);
    }

    #[test]
    fn test_julia_iterations() {
        let iterations = FractalType::Julia.iterations(0.0, 0.0, 1000, &Point::new(0.355, 0.355));
        assert!(iterations > 0);
    }

    #[test]
    fn test_burning_ship_iterations() {
        let iterations = FractalType::BurningShip.iterations(0.0, 0.0, 1000, &Point::new(0.0, 0.0));
        assert!(iterations > 0);
    }

    #[test]
    fn test_tricorn_iterations() {
        let iterations = FractalType::Tricorn.iterations(0.0, 0.0, 1000, &Point::new(0.0, 0.0));
        assert!(iterations > 0);
    }
}
