use eframe::epaint::Color32;

#[derive(Clone, Copy, PartialEq)]
pub enum ColorScheme {
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

    fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).max(0.0).min(1.0);
        t * t * (3.0 - 2.0 * t)
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }

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
    pub fn to_color32(&self, iterations: u16, max_iterations: u16) -> Color32 {
        if iterations >= max_iterations {
            return Color32::BLACK;
        }

        let t: f32 = iterations as f32 / max_iterations as f32;
        let smoothed: f32 = t.powf(0.5);

        match self {
            ColorScheme::UltraSmooth => {
                let phase: f32 = smoothed * 6.28318;
                let r: u8 = (128.0 + 127.0 * (phase).sin()) as u8;
                let g: u8 = (128.0 + 127.0 * (phase + 2.094).sin()) as u8;
                let b: u8 = (128.0 + 127.0 * (phase + 4.188).sin()) as u8;
                Color32::from_rgb(r, g, b)
            }

            ColorScheme::DeepOcean => {
                // Smooth ocean depths with enhanced gradients
                let depth: f32 = smoothed.powf(1.5);
                let wave: f32 = (smoothed * 8.0 * std::f32::consts::PI).sin() * 0.1 + 1.0;

                let r: u8 = (10.0 + 45.0 * depth * wave) as u8;
                let g: u8 = (20.0 + 150.0 * Self::smooth_step(0.0, 1.0, depth)) as u8;
                let b: u8 = (50.0 + 205.0 * Self::smooth_step(0.2, 1.0, depth)) as u8;
                Color32::from_rgb(r, g, b)
            }

            ColorScheme::PrismaticFire => {
                // Multi-layered fire effect with smooth transitions
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

            ColorScheme::AuroralDream => {
                let wave1: f32 = (smoothed * 3.0 * std::f32::consts::PI).sin();
                let wave2: f32 = (smoothed * 5.0 * std::f32::consts::PI + 1.0).sin();
                let wave3: f32 = (smoothed * 7.0 * std::f32::consts::PI + 2.0).sin();

                let r: u8 =
                    (50.0 + 100.0 * (0.5 + 0.3 * wave1 + 0.2 * wave3).max(0.0).min(1.0)) as u8;
                let g: u8 = (100.0 + 155.0 * (0.7 + 0.3 * wave2).max(0.0).min(1.0)) as u8;
                let b: u8 = (80.0 + 175.0 * (0.6 + 0.4 * wave1 * wave2).max(0.0).min(1.0)) as u8;
                Color32::from_rgb(r, g, b)
            }

            ColorScheme::CosmicNebula => {
                let cosmic_t: f32 = smoothed.powf(0.6);
                let dust_pattern: f32 = (cosmic_t * 4.0 * std::f32::consts::PI).sin().abs();
                let gas_pattern: f32 = (cosmic_t * 6.0 * std::f32::consts::PI + 1.5).cos().abs();

                let r: u8 = (80.0 + 175.0 * Self::lerp(dust_pattern, gas_pattern, cosmic_t)) as u8;
                let g: u8 = (40.0 + 150.0 * cosmic_t * dust_pattern) as u8;
                let b: u8 = (120.0 + 135.0 * gas_pattern * cosmic_t.powf(0.5)) as u8;
                Color32::from_rgb(r, g, b)
            }

            ColorScheme::RainbowSmooth => {
                let hue: f32 = smoothed * 360.0;
                let saturation: f32 =
                    0.8 + 0.2 * (smoothed * 2.0 * std::f32::consts::PI).sin().abs();
                let value: f32 = 0.9 + 0.1 * (smoothed * 3.0 * std::f32::consts::PI).cos().abs();

                Self:: hsv_to_rgb(hue, saturation, value)
            }

            ColorScheme::VelvetShadow => {
                let depth: f32 = Self::smooth_step(0.0, 1.0, smoothed);
                let texture: f32 = (smoothed * 10.0 * std::f32::consts::PI).sin() * 0.08 + 1.0;

                let r: u8 = (20.0 + 120.0 * depth.powf(2.0) * texture) as u8;
                let g: u8 = (10.0 + 80.0 * depth.powf(1.5)) as u8;
                let b: u8 = (40.0 + 180.0 * depth * texture) as u8;
                Color32::from_rgb(r, g, b)
            }

            ColorScheme::GoldenHour => {
                let warmth: f32 = Self::smooth_step(0.0, 1.0, smoothed);
                let glow: f32 = (smoothed * 4.0 * std::f32::consts::PI).sin().abs() * 0.1 + 0.9;

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
            ColorScheme::Classic => {
                let r:u8 = (255.0 * (0.5 + 0.5 * (4.0 * smoothed).sin())) as u8;
                let g:u8 = (255.0 * (0.5 + 0.5 * (2.0 * smoothed + 2.0).sin())) as u8;
                let b:u8 = (255.0 * (1.0 - smoothed).powf(0.3)) as u8;
                Color32::from_rgb(r, g, b)
            }
            ColorScheme::Hot => {
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
            ColorScheme::Cool => {
                // Ocean-inspired cool palette
                let r:u8 = (100.0 * (1.0 - smoothed).powf(2.0)) as u8;
                let g:u8 = (50.0 + 205.0 * smoothed.powf(0.7)) as u8;
                let b:u8 = (150.0 + 105.0 * smoothed) as u8;
                Color32::from_rgb(r, g, b)
            }
            ColorScheme::Psychedelic => {
                let angle: f32 = smoothed * 6.28318 * 3.0;
                let r:u8 = (127.5 + 127.5 * angle.sin()) as u8;
                let g:u8 = (127.5 + 127.5 * (angle + 2.094).sin()) as u8;
                let b:u8 = (127.5 + 127.5 * (angle + 4.188).sin()) as u8;
                Color32::from_rgb(r, g, b)
            }
            ColorScheme::Sunset => {
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
            ColorScheme::Electric => {
                let pulse = (smoothed * 10.0).sin().abs();
                let r = (255.0 * (smoothed * 2.0).min(1.0) * pulse) as u8;
                let g = (255.0 * ((smoothed - 0.2) * 2.5).max(0.0).min(1.0)) as u8;
                let b = (255.0 * ((1.0 - smoothed) * 1.5).min(1.0)) as u8;
                Color32::from_rgb(r, g, b)
            }
            ColorScheme::Forest => {
                if smoothed < 0.4 {
                    let t = smoothed / 0.4;
                    let r = (20.0 + 80.0 * t) as u8;
                    let g = (40.0 + 120.0 * t) as u8;
                    let b = (10.0 + 30.0 * t) as u8;
                    Color32::from_rgb(r, g, b)
                } else {
                    let t = (smoothed - 0.4) / 0.6;
                    let r = (100.0 + 155.0 * t) as u8;
                    let g = (160.0 + 95.0 * t) as u8;
                    let b = (40.0 + 60.0 * t) as u8;
                    Color32::from_rgb(r, g, b)
                }
            }
            ColorScheme::Galaxy => {
                let cycle = (smoothed * 4.0 * std::f32::consts::PI).sin().abs();
                let r = (50.0 + 150.0 * cycle * smoothed) as u8;
                let g = (20.0 + 100.0 * (1.0 - smoothed).powf(0.5)) as u8;
                let b = (100.0 + 155.0 * smoothed.powf(0.3)) as u8;
                Color32::from_rgb(r, g, b)
            }
            ColorScheme::Grayscale => {
                let gray = (255.0 * smoothed.powf(0.8)) as u8;
                Color32::from_rgb(gray, gray, gray)
            }
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme::Classic
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
