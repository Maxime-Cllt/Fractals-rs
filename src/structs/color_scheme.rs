use eframe::epaint::Color32;
use std::f32::consts::PI;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ColorScheme {
    #[default]
    Classic,
    Hot,
    Cool,
    Psychedelic,
    Sunset,
    Electric,
    Forest,
    Galaxy,
    Grayscale,
    UltraSmooth,
    DeepOcean,
    PrismaticFire,
    AuroralDream,
    CosmicNebula,
    RainbowSmooth,
    VelvetShadow,
    GoldenHour,
}

impl ColorScheme {
    /// Returns the name of the color scheme.
    #[inline]
    pub fn name(&self) -> &'static str {
        match self {
            ColorScheme::Classic => "Classic",
            ColorScheme::Hot => "Hot",
            ColorScheme::Cool => "Cool",
            ColorScheme::Grayscale => "Grayscale",
            ColorScheme::Psychedelic => "Psychedelic",
            ColorScheme::Sunset => "Sunset",
            ColorScheme::Electric => "Electric",
            ColorScheme::Forest => "Forest",
            ColorScheme::Galaxy => "Galaxy",
            ColorScheme::UltraSmooth => "Ultra Smooth",
            ColorScheme::DeepOcean => "Deep Ocean",
            ColorScheme::PrismaticFire => "Prismatic Fire",
            ColorScheme::AuroralDream => "Auroral Dream",
            ColorScheme::CosmicNebula => "Cosmic Nebula",
            ColorScheme::RainbowSmooth => "Rainbow Smooth",
            ColorScheme::VelvetShadow => "Velvet Shadow",
            ColorScheme::GoldenHour => "Golden Hour",
        }
    }

    /// Returns all available color schemes.
    #[inline]
    pub const fn all() -> [ColorScheme; 17] {
        [
            ColorScheme::Classic,
            ColorScheme::Hot,
            ColorScheme::Cool,
            ColorScheme::Grayscale,
            ColorScheme::Psychedelic,
            ColorScheme::Sunset,
            ColorScheme::Electric,
            ColorScheme::Forest,
            ColorScheme::Galaxy,
            ColorScheme::UltraSmooth,
            ColorScheme::DeepOcean,
            ColorScheme::PrismaticFire,
            ColorScheme::AuroralDream,
            ColorScheme::CosmicNebula,
            ColorScheme::RainbowSmooth,
            ColorScheme::VelvetShadow,
            ColorScheme::GoldenHour,
        ]
    }

    /// Smooth step function for smooth interpolation between two edges.
    #[inline]
    fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Performs linear interpolation between two values.
    #[inline]
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }

    /// Converts HSV color to RGB.
    #[inline]
    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color32 {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r_prime, g_prime, b_prime) = match h as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        let r = ((r_prime + m) * 255.0) as u8;
        let g = ((g_prime + m) * 255.0) as u8;
        let b = ((b_prime + m) * 255.0) as u8;

        Color32::from_rgb(r, g, b)
    }
}

impl ColorScheme {
    /// Converts the number of iterations to a color based on the color scheme.
    #[inline]
    pub fn to_color32(&self, iterations: u16, max_iterations: u16) -> Color32 {
        if iterations >= max_iterations {
            return Color32::BLACK;
        }

        let t: f32 = iterations as f32 / max_iterations as f32;
        let smoothed: f32 = t.powf(0.5);

        match self {
            Self::Classic => {
                let r: u8 = (255.0 * (0.5 + 0.5 * (4.0 * smoothed).sin())) as u8;
                let g: u8 = (255.0 * (0.5 + 0.5 * (2.0 * smoothed + 2.0).sin())) as u8;
                let b: u8 = (255.0 * (1.0 - smoothed).powf(0.3)) as u8;
                Color32::from_rgb(r, g, b)
            }
            Self::Hot => {
                if smoothed < 0.25 {
                    let t: f32 = smoothed * 4.0;
                    Color32::from_rgb((80.0 + 175.0 * t) as u8, (20.0 * t) as u8, 0)
                } else if smoothed < 0.5 {
                    let t: f32 = (smoothed - 0.25) * 4.0;
                    Color32::from_rgb(255, (20.0 + 235.0 * t) as u8, 0)
                } else if smoothed < 0.75 {
                    let t: f32 = (smoothed - 0.5) * 4.0;
                    Color32::from_rgb(255, 255, (200.0 * t) as u8)
                } else {
                    let t: f32 = (smoothed - 0.75) * 4.0;
                    Color32::from_rgb(255, 255, (200.0 + 55.0 * t) as u8)
                }
            }
            Self::Cool => {
                let ice_shimmer = (smoothed * 6.0 * PI).sin() * 0.3 + 0.7;
                let frost_pattern = (smoothed * 4.0 * PI).cos().abs();

                let r = (60.0 + 95.0 * (1.0 - smoothed).powf(1.5) * ice_shimmer) as u8;
                let g = (120.0 + 135.0 * smoothed.powf(0.4) * frost_pattern) as u8;
                let b = (180.0 + 75.0 * smoothed.powf(0.3)) as u8;
                Color32::from_rgb(r, g, b)
            }
            Self::Psychedelic => {
                let angle: f32 = smoothed * (2.0 * PI) * 3.0;
                let r: u8 = (127.5 + 127.5 * angle.sin()) as u8;
                let g: u8 = (127.5 + 127.5 * (angle + 2.094).sin()) as u8;
                let b: u8 = (127.5 + 127.5 * (angle + 4.188).sin()) as u8;
                Color32::from_rgb(r, g, b)
            }
            Self::Sunset => {
                if smoothed < 0.3 {
                    let t = smoothed / 0.3;
                    let r = (30.0 + 225.0 * t) as u8;
                    let g = (0.0 + 50.0 * t) as u8;
                    let b = (80.0 * (1.0 - t)) as u8;
                    Color32::from_rgb(r, g, b)
                } else if smoothed < 0.7 {
                    let t = (smoothed - 0.3) / 0.4;
                    let r = (255.0 - 55.0 * t) as u8;
                    let g = (50.0 + 155.0 * t) as u8;
                    let b = (20.0 + 80.0 * t) as u8;
                    Color32::from_rgb(r, g, b)
                } else {
                    let t = (smoothed - 0.7) / 0.3;
                    let r = (200.0 + 55.0 * t) as u8;
                    let g = (205.0 + 50.0 * t) as u8;
                    let b = (100.0 + 155.0 * t) as u8;
                    Color32::from_rgb(r, g, b)
                }
            }
            Self::Electric => {
                let pulse = (smoothed * 10.0).sin().abs();
                let r = (255.0 * (smoothed * 2.0).min(1.0) * pulse) as u8;
                let g = ((smoothed - 0.2) * 2.5).clamp(0.0, 1.0) as u8;
                let b = (255.0 * ((1.0 - smoothed) * 1.5).min(1.0)) as u8;
                Color32::from_rgb(r, g, b)
            }
            Self::Forest => {
                let hue_wave = (smoothed * 4.0 * PI).sin() * 0.5 + 0.5;
                let depth_wave = (smoothed * 6.0 * PI).cos().abs();

                if smoothed < 0.3 {
                    // Forest depths
                    let t = smoothed / 0.3;
                    let r = (10.0 + 35.0 * t * hue_wave) as u8;
                    let g = (20.0 + 60.0 * t) as u8;
                    let b = (8.0 + 25.0 * t * depth_wave) as u8;
                    Color32::from_rgb(r, g, b)
                } else if smoothed < 0.7 {
                    // Mid forest
                    let t = (smoothed - 0.3) / 0.4;
                    let r = (45.0 + 80.0 * t * hue_wave) as u8;
                    let g = (80.0 + 100.0 * t) as u8;
                    let b = (25.0 + 35.0 * t * (1.0 - hue_wave * 0.6)) as u8;
                    Color32::from_rgb(r, g, b)
                } else {
                    // Sunlit canopy
                    let t = (smoothed - 0.7) / 0.3;
                    let golden_hour = (hue_wave * depth_wave).powf(0.5);
                    let r = (125.0 + 130.0 * t * golden_hour) as u8;
                    let g = (180.0 + 75.0 * t) as u8;
                    let b = (60.0 + 40.0 * t * (1.0 - golden_hour * 0.8)) as u8;
                    Color32::from_rgb(r, g, b)
                }
            }
            Self::Galaxy => {
                let cycle = (smoothed * 3.0 * PI).sin().abs();
                let spiral = (smoothed * 6.0 * PI).cos() * 0.5 + 0.5;
                let r = (60.0 + 140.0 * (cycle * smoothed).powf(0.8)) as u8;
                let g = (15.0 + 60.0 * (1.0 - smoothed).powf(0.6)) as u8;
                let b = (120.0 + 135.0 * (smoothed * spiral).powf(0.5)) as u8;
                Color32::from_rgb(r, g, b)
            }
            Self::Grayscale => {
                let gray = (255.0 * smoothed.powf(0.8)) as u8;
                Color32::from_rgb(gray, gray, gray)
            }

            Self::UltraSmooth => {
                let phase: f32 = smoothed * (2.0 * PI);
                let r: u8 = 127.0f32.mul_add(phase.sin(), 128.0) as u8;
                let g: u8 = 127.0f32.mul_add((phase + 2.094).sin(), 128.0) as u8;
                let b: u8 = 127.0f32.mul_add((phase + 4.188).sin(), 128.0) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::DeepOcean => {
                let depth: f32 = smoothed.powf(1.5);
                let wave: f32 = (smoothed * 8.0 * PI).sin() * 0.1 + 1.0;

                let r: u8 = (10.0 + 45.0 * depth * wave) as u8;
                let g: u8 = (20.0 + 150.0 * Self::smooth_step(0.0, 1.0, depth)) as u8;
                let b: u8 = (50.0 + 205.0 * Self::smooth_step(0.2, 1.0, depth)) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::PrismaticFire => {
                let heat = smoothed.powf(0.7);
                let flicker = (smoothed * 12.0).sin() * 0.05 + 1.0;

                if heat < 0.2 {
                    let t: f32 = heat * 5.0;
                    let r: u8 = (80.0 + 175.0 * Self::smooth_step(0.0, 1.0, t) * flicker) as u8;
                    let g: u8 = (0.0 + 30.0 * t * t) as u8;
                    let b: u8 = (0.0 + 15.0 * t) as u8;
                    Color32::from_rgb(r, g, b)
                } else if heat < 0.5 {
                    let t: f32 = (heat - 0.2) / 0.3;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = (255.0 * flicker) as u8;
                    let g: u8 = (30.0 + 195.0 * smooth_t * flicker) as u8;
                    let b: u8 = (15.0 + 35.0 * smooth_t) as u8;
                    Color32::from_rgb(r, g, b)
                } else if heat < 0.8 {
                    let t: f32 = (heat - 0.5) / 0.3;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = 255;
                    let g: u8 = (225.0 + 30.0 * smooth_t) as u8;
                    let b: u8 = (50.0 + 150.0 * smooth_t * flicker) as u8;
                    Color32::from_rgb(r, g, b)
                } else {
                    let t: f32 = (heat - 0.8) / 0.2;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = 255;
                    let g: u8 = 255;
                    let b: u8 = (200.0 + 55.0 * smooth_t) as u8;
                    Color32::from_rgb(r, g, b)
                }
            }

            Self::AuroralDream => {
                let wave1: f32 = (smoothed * 3.0 * PI).sin();
                let wave2: f32 = (smoothed * 5.0 * PI + 1.0).sin();
                let wave3: f32 = (smoothed * 7.0 * PI + 2.0).sin();

                let r: u8 =
                    (50.0 + 100.0 * (0.5 + 0.3 * wave1 + 0.2 * wave3).clamp(0.0, 1.0)) as u8;
                let g: u8 = (100.0 + 155.0 * (0.7 + 0.3 * wave2).clamp(0.0, 1.0)) as u8;
                let b: u8 = (80.0 + 175.0 * (0.6 + 0.4 * wave1 * wave2).clamp(0.0, 1.0)) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::CosmicNebula => {
                let cosmic_t: f32 = smoothed.powf(0.6);
                let dust_pattern: f32 = (cosmic_t * 4.0 * PI).sin().abs();
                let gas_pattern: f32 = (cosmic_t * 6.0 * PI + 1.5).cos().abs();

                let r: u8 = (80.0 + 175.0 * Self::lerp(dust_pattern, gas_pattern, cosmic_t)) as u8;
                let g: u8 = (40.0 + 150.0 * cosmic_t * dust_pattern) as u8;
                let b: u8 = (120.0 + 135.0 * gas_pattern * cosmic_t.powf(0.5)) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::RainbowSmooth => {
                let hue: f32 = smoothed * 360.0;
                let saturation: f32 = 0.8 + 0.2 * (smoothed * 2.0 * PI).sin().abs();
                let value: f32 = 0.9 + 0.1 * (smoothed * 3.0 * PI).cos().abs();

                Self::hsv_to_rgb(hue, saturation, value)
            }

            Self::VelvetShadow => {
                let depth: f32 = Self::smooth_step(0.0, 1.0, smoothed);
                let texture: f32 = (smoothed * 10.0 * PI).sin() * 0.08 + 1.0;

                let r: u8 = (20.0 + 120.0 * depth.powf(2.0) * texture) as u8;
                let g: u8 = (10.0 + 80.0 * depth.powf(1.5)) as u8;
                let b: u8 = (40.0 + 180.0 * depth * texture) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::GoldenHour => {
                let warmth: f32 = Self::smooth_step(0.0, 1.0, smoothed);
                let glow: f32 = (smoothed * 4.0 * PI).sin().abs() * 0.1 + 0.9;

                if warmth < 0.4 {
                    let t: f32 = warmth / 0.4;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = (40.0 + 180.0 * smooth_t * glow) as u8;
                    let g: u8 = (20.0 + 100.0 * smooth_t * glow) as u8;
                    let b: u8 = (80.0 * (1.0 - smooth_t)) as u8;
                    Color32::from_rgb(r, g, b)
                } else if warmth < 0.8 {
                    let t: f32 = (warmth - 0.4) / 0.4;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = (220.0 + 35.0 * smooth_t * glow) as u8;
                    let g: u8 = (120.0 + 100.0 * smooth_t * glow) as u8;
                    let b: u8 = (30.0 + 70.0 * smooth_t) as u8;
                    Color32::from_rgb(r, g, b)
                } else {
                    let t: f32 = (warmth - 0.8) / 0.2;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = 255;
                    let g: u8 = (220.0 + 35.0 * smooth_t) as u8;
                    let b: u8 = (100.0 + 100.0 * smooth_t * glow) as u8;
                    Color32::from_rgb(r, g, b)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_names() {
        assert_eq!(ColorScheme::Classic.name(), "Classic");
        assert_eq!(ColorScheme::Hot.name(), "Hot");
        assert_eq!(ColorScheme::Cool.name(), "Cool");
        assert_eq!(ColorScheme::Grayscale.name(), "Grayscale");
        assert_eq!(ColorScheme::Psychedelic.name(), "Psychedelic");
        assert_eq!(ColorScheme::Sunset.name(), "Sunset");
        assert_eq!(ColorScheme::Electric.name(), "Electric");
        assert_eq!(ColorScheme::Forest.name(), "Forest");
        assert_eq!(ColorScheme::Galaxy.name(), "Galaxy");
    }

    #[test]
    fn test_hsv_to_rgb() {
        let color = ColorScheme::hsv_to_rgb(0.0, 1.0, 1.0);
        assert_eq!(color, Color32::from_rgb(255, 0, 0)); // Red

        let color = ColorScheme::hsv_to_rgb(120.0, 1.0, 1.0);
        assert_eq!(color, Color32::from_rgb(0, 255, 0)); // Green

        let color = ColorScheme::hsv_to_rgb(240.0, 1.0, 1.0);
        assert_eq!(color, Color32::from_rgb(0, 0, 255)); // Blue
    }

    #[test]
    fn test_smooth_step() {
        assert_eq!(ColorScheme::smooth_step(0.0, 1.0, 0.0), 0.0);
        assert_eq!(ColorScheme::smooth_step(0.0, 1.0, 1.0), 1.0);
        assert!((ColorScheme::smooth_step(0.0, 1.0, 0.5) - 0.5).abs() < 0.01);
        assert!((ColorScheme::smooth_step(0.2, 0.8, 0.5) - 0.5).abs() < 0.01);
    }
}
