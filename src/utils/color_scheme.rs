use eframe::epaint::Color32;
use std::f32::consts::PI;

#[derive(Clone, Debug, Copy, PartialEq, Default)]
#[repr(u8)]
/// Enum representing different color schemes for fractal rendering.
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
    MoltenLava,
    IcebergGlacier,
    NorthernLights,
    TropicalParadise,
    VaporwaveNeon,
    MidnightStars,
    CherryBlossom,
    QuantumPlasma,
}

impl ColorScheme {
    /// Returns the name of the color scheme.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::Hot => "Hot",
            Self::Cool => "Cool",
            Self::Grayscale => "Grayscale",
            Self::Psychedelic => "Psychedelic",
            Self::Sunset => "Sunset",
            Self::Electric => "Electric",
            Self::Forest => "Forest",
            Self::Galaxy => "Galaxy",
            Self::UltraSmooth => "Ultra Smooth",
            Self::DeepOcean => "Deep Ocean",
            Self::PrismaticFire => "Prismatic Fire",
            Self::AuroralDream => "Auroral Dream",
            Self::CosmicNebula => "Cosmic Nebula",
            Self::RainbowSmooth => "Rainbow Smooth",
            Self::VelvetShadow => "Velvet Shadow",
            Self::GoldenHour => "Golden Hour",
            Self::MoltenLava => "Molten Lava",
            Self::IcebergGlacier => "Iceberg Glacier",
            Self::NorthernLights => "Northern Lights",
            Self::TropicalParadise => "Tropical Paradise",
            Self::VaporwaveNeon => "Vaporwave Neon",
            Self::MidnightStars => "Midnight Stars",
            Self::CherryBlossom => "Cherry Blossom",
            Self::QuantumPlasma => "Quantum Plasma",
        }
    }

    /// Returns all available color schemes.
    #[inline]
    pub const fn all() -> [Self; 25] {
        [
            Self::Classic,
            Self::Hot,
            Self::Cool,
            Self::Grayscale,
            Self::Psychedelic,
            Self::Sunset,
            Self::Electric,
            Self::Forest,
            Self::Galaxy,
            Self::UltraSmooth,
            Self::DeepOcean,
            Self::PrismaticFire,
            Self::AuroralDream,
            Self::CosmicNebula,
            Self::RainbowSmooth,
            Self::VelvetShadow,
            Self::GoldenHour,
            Self::MoltenLava,
            Self::IcebergGlacier,
            Self::NorthernLights,
            Self::TropicalParadise,
            Self::VaporwaveNeon,
            Self::MidnightStars,
            Self::CherryBlossom,
            Self::QuantumPlasma,
        ]
    }

    /// Smooth step function for smooth interpolation between two edges.
    #[inline]
    fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * 2.0f32.mul_add(-t, 3.0)
    }

    /// Performs linear interpolation between two values.
    #[inline]
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        t.mul_add(b - a, a)
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
    #[must_use]
    pub fn to_color32(&self, iterations: u16, max_iterations: u16) -> Color32 {
        if iterations >= max_iterations {
            return Color32::BLACK;
        }

        let t: f32 = f32::from(iterations) / f32::from(max_iterations);
        let smoothed: f32 = t.sqrt();

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
                let b: u8 = 205.0f32.mul_add(Self::smooth_step(0.2, 1.0, depth), 50.0) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::PrismaticFire => {
                let heat = smoothed.powf(0.7);
                let flicker = (smoothed * 12.0).sin() * 0.05 + 1.0;

                if heat < 0.2 {
                    let t: f32 = heat * 5.0;
                    let r: u8 =
                        (175.0 * Self::smooth_step(0.0, 1.0, t)).mul_add(flicker, 80.0) as u8;
                    let g: u8 = (30.0 * t).mul_add(t, 0.0) as u8;
                    let b: u8 = 15.0f32.mul_add(t, 0.0) as u8;
                    Color32::from_rgb(r, g, b)
                } else if heat < 0.5 {
                    let t: f32 = (heat - 0.2) / 0.3;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = (255.0 * flicker) as u8;
                    let g: u8 = (195.0 * smooth_t).mul_add(flicker, 30.0) as u8;
                    let b: u8 = 35.0f32.mul_add(smooth_t, 15.0) as u8;
                    Color32::from_rgb(r, g, b)
                } else if heat < 0.8 {
                    let t: f32 = (heat - 0.5) / 0.3;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = 255;
                    let g: u8 = 30.0f32.mul_add(smooth_t, 225.0) as u8;
                    let b: u8 = (150.0 * smooth_t).mul_add(flicker, 50.0) as u8;
                    Color32::from_rgb(r, g, b)
                } else {
                    let t: f32 = (heat - 0.8) / 0.2;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = 255;
                    let g: u8 = 255;
                    let b: u8 = 55.0f32.mul_add(smooth_t, 200.0) as u8;
                    Color32::from_rgb(r, g, b)
                }
            }

            Self::AuroralDream => {
                let wave1: f32 = (smoothed * 3.0 * PI).sin();
                let wave2: f32 = (smoothed * 5.0).mul_add(PI, 1.0).sin();
                let wave3: f32 = (smoothed * 7.0).mul_add(PI, 2.0).sin();

                let r: u8 = (50.0
                    + 100.0
                        * 0.2f32
                            .mul_add(wave3, 0.3f32.mul_add(wave1, 0.5))
                            .clamp(0.0, 1.0)) as u8;
                let g: u8 =
                    155.0f32.mul_add(0.3f32.mul_add(wave2, 0.7).clamp(0.0, 1.0), 100.0) as u8;
                let b: u8 =
                    175.0f32.mul_add((0.4 * wave1).mul_add(wave2, 0.6).clamp(0.0, 1.0), 80.0) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::CosmicNebula => {
                let cosmic_t: f32 = smoothed.powf(0.6);
                let dust_pattern: f32 = (cosmic_t * 4.0 * PI).sin().abs();
                let gas_pattern: f32 = (cosmic_t * 6.0).mul_add(PI, 1.5).cos().abs();

                let r: u8 =
                    175.0f32.mul_add(Self::lerp(dust_pattern, gas_pattern, cosmic_t), 80.0) as u8;
                let g: u8 = (150.0 * cosmic_t).mul_add(dust_pattern, 40.0) as u8;
                let b: u8 = (135.0 * gas_pattern).mul_add(cosmic_t.sqrt(), 120.0) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::RainbowSmooth => {
                let hue: f32 = smoothed * 360.0;
                let saturation: f32 = 0.2f32.mul_add((smoothed * 2.0 * PI).sin().abs(), 0.8);
                let value: f32 = 0.1f32.mul_add((smoothed * 3.0 * PI).cos().abs(), 0.9);

                Self::hsv_to_rgb(hue, saturation, value)
            }

            Self::VelvetShadow => {
                let depth: f32 = Self::smooth_step(0.0, 1.0, smoothed);
                let texture: f32 = (smoothed * 10.0 * PI).sin().mul_add(0.08, 1.0);

                let r: u8 = (120.0 * depth.powi(2)).mul_add(texture, 20.0) as u8;
                let g: u8 = 80.0f32.mul_add(depth.powf(1.5), 10.0) as u8;
                let b: u8 = (180.0 * depth).mul_add(texture, 40.0) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::GoldenHour => {
                let warmth: f32 = Self::smooth_step(0.0, 1.0, smoothed);
                let glow: f32 = (smoothed * 4.0 * PI).sin().abs().mul_add(0.1, 0.9);

                if warmth < 0.4 {
                    let t: f32 = warmth / 0.4;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = (180.0 * smooth_t).mul_add(glow, 40.0) as u8;
                    let g: u8 = (100.0 * smooth_t).mul_add(glow, 20.0) as u8;
                    let b: u8 = (80.0 * (1.0 - smooth_t)) as u8;
                    Color32::from_rgb(r, g, b)
                } else if warmth < 0.8 {
                    let t: f32 = (warmth - 0.4) / 0.4;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = (35.0 * smooth_t).mul_add(glow, 220.0) as u8;
                    let g: u8 = (100.0 * smooth_t).mul_add(glow, 120.0) as u8;
                    let b: u8 = 70.0f32.mul_add(smooth_t, 30.0) as u8;
                    Color32::from_rgb(r, g, b)
                } else {
                    let t: f32 = (warmth - 0.8) / 0.2;
                    let smooth_t: f32 = Self::smooth_step(0.0, 1.0, t);
                    let r: u8 = 255;
                    let g: u8 = 35.0f32.mul_add(smooth_t, 220.0) as u8;
                    let b: u8 = (100.0 * smooth_t).mul_add(glow, 100.0) as u8;
                    Color32::from_rgb(r, g, b)
                }
            }

            Self::MoltenLava => {
                // Ultra-realistic lava with heat distortion
                let heat = smoothed.powf(0.6);
                let turbulence = (smoothed * 15.0 * PI).sin() * 0.08 + 1.0;
                let core_temp = (smoothed * 7.0 * PI).cos().abs() * 0.15 + 0.85;

                if heat < 0.15 {
                    // Deep volcanic rock - almost black with hint of red
                    let t = heat / 0.15;
                    let r = (90.0 * t * core_temp) as u8;
                    let g = (15.0 * t) as u8;
                    let b = (5.0 * t) as u8;
                    Color32::from_rgb(r, g, b)
                } else if heat < 0.4 {
                    // Heating rock - dark red to orange
                    let t = (heat - 0.15) / 0.25;
                    let smooth_t = Self::smooth_step(0.0, 1.0, t);
                    let r = (165.0 * smooth_t * turbulence + 90.0) as u8;
                    let g = (50.0 * smooth_t + 15.0) as u8;
                    let b = (10.0 * smooth_t + 5.0) as u8;
                    Color32::from_rgb(r, g, b)
                } else if heat < 0.7 {
                    // Molten lava - bright orange to yellow
                    let t = (heat - 0.4) / 0.3;
                    let smooth_t = Self::smooth_step(0.0, 1.0, t);
                    let r = (255.0 * turbulence).min(255.0) as u8;
                    let g = (140.0 * smooth_t * turbulence + 65.0) as u8;
                    let b = (25.0 * smooth_t + 15.0) as u8;
                    Color32::from_rgb(r, g, b)
                } else {
                    // White-hot core - extreme heat
                    let t = (heat - 0.7) / 0.3;
                    let smooth_t = Self::smooth_step(0.0, 1.0, t);
                    let r = 255;
                    let g = (50.0 * smooth_t + 205.0) as u8;
                    let b = (180.0 * smooth_t * core_temp + 40.0) as u8;
                    Color32::from_rgb(r, g, b)
                }
            }

            Self::IcebergGlacier => {
                // Crystalline ice with depth and refraction
                let depth = smoothed.powf(1.2);
                let crystal = (smoothed * 8.0 * PI).sin().abs();
                let refraction = (smoothed * 12.0 * PI).cos() * 0.5 + 0.5;
                let shimmer = (smoothed * 20.0 * PI).sin() * 0.1 + 0.9;

                let base_cyan = 150.0 + 105.0 * depth * shimmer;
                let ice_blue = 180.0 + 75.0 * (1.0 - depth).powf(0.5) * crystal;
                let highlight = 200.0 + 55.0 * refraction * (1.0 - depth);

                let r = (base_cyan * 0.7 * refraction) as u8;
                let g = ice_blue as u8;
                let b = highlight as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::NorthernLights => {
                // Aurora Borealis - flowing ethereal lights
                let flow = smoothed.powf(0.8);
                let wave1 = (flow * 4.0 * PI).sin();
                let wave2 = (flow * 6.0 * PI + 1.5).sin();
                let wave3 = (flow * 3.0 * PI + 3.0).sin();
                let shimmer = (flow * 15.0 * PI).cos().abs() * 0.2 + 0.8;

                // Mix of green, blue, and magenta aurora
                let green_aurora = (0.5 + 0.5 * wave1) * shimmer;
                let blue_aurora = (0.5 + 0.5 * wave2) * shimmer;
                let magenta_aurora = (0.5 + 0.5 * wave3).powf(2.0) * shimmer;

                let r = (80.0 + 120.0 * magenta_aurora) as u8;
                let g = (100.0 + 155.0 * green_aurora) as u8;
                let b = (120.0 + 135.0 * blue_aurora) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::TropicalParadise => {
                // Vibrant tropical colors - ocean to sunset
                let paradise = smoothed.powf(0.7);
                let wave = (paradise * 5.0 * PI).sin() * 0.5 + 0.5;
                let bloom = (paradise * 3.0 * PI).cos().abs();

                if paradise < 0.3 {
                    // Deep ocean turquoise
                    let t = paradise / 0.3;
                    let r = (30.0 + 50.0 * t * wave) as u8;
                    let g = (120.0 + 80.0 * t) as u8;
                    let b = (150.0 + 70.0 * t * bloom) as u8;
                    Color32::from_rgb(r, g, b)
                } else if paradise < 0.6 {
                    // Tropical cyan to mint
                    let t = (paradise - 0.3) / 0.3;
                    let r = (80.0 + 100.0 * t * bloom) as u8;
                    let g = (200.0 + 35.0 * t) as u8;
                    let b = (220.0 - 40.0 * t * wave) as u8;
                    Color32::from_rgb(r, g, b)
                } else {
                    // Sunset coral and pink
                    let t = (paradise - 0.6) / 0.4;
                    let r = (180.0 + 75.0 * t) as u8;
                    let g = (235.0 - 50.0 * t * wave) as u8;
                    let b = (180.0 + 50.0 * t * bloom) as u8;
                    Color32::from_rgb(r, g, b)
                }
            }

            Self::VaporwaveNeon => {
                // 80s/90s aesthetic with neon colors
                let vibe = smoothed.powf(0.75);
                let grid = ((vibe * 20.0).fract() * 2.0 - 1.0).abs();
                let glow = (vibe * 6.0 * PI).sin().abs();
                let pulse = (vibe * 10.0 * PI).sin() * 0.15 + 0.85;

                // Neon pink, cyan, and purple
                let neon_pink = (0.7 + 0.3 * glow) * pulse;
                let neon_cyan = (0.6 + 0.4 * (1.0 - glow)) * pulse;
                let neon_purple = (0.5 + 0.5 * grid) * pulse;

                let r = (150.0 + 105.0 * neon_pink) as u8;
                let g = (80.0 + 120.0 * neon_cyan) as u8;
                let b = (180.0 + 75.0 * neon_purple) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::MidnightStars => {
                // Deep space with stars and nebula
                let space = smoothed.powf(1.5);
                let stars = (space * 50.0 * PI).sin();
                let star_brightness = if stars > 0.95 {
                    (stars - 0.95) * 20.0
                } else {
                    0.0
                };
                let nebula = (space * 3.0 * PI).sin().abs();
                let galaxy_dust = (space * 8.0 * PI).cos() * 0.5 + 0.5;

                let r = (10.0 + 30.0 * nebula + 245.0 * star_brightness) as u8;
                let g = (5.0 + 20.0 * galaxy_dust + 245.0 * star_brightness) as u8;
                let b = (30.0 + 80.0 * space.sqrt() + 245.0 * star_brightness) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::CherryBlossom => {
                // Delicate pink and white spring blossoms
                let bloom = smoothed.powf(0.6);
                let petal = (bloom * 6.0 * PI).sin() * 0.5 + 0.5;
                let breeze = (bloom * 4.0 * PI).cos().abs();
                let soft_light = Self::smooth_step(0.0, 1.0, bloom);

                let pink_intensity = petal * soft_light;
                let white_highlight = (1.0 - bloom * 0.5) * breeze;

                let r = (200.0 + 55.0 * pink_intensity) as u8;
                let g = (150.0 + 70.0 * white_highlight) as u8;
                let b = (180.0 + 40.0 * pink_intensity - 50.0 * white_highlight) as u8;
                Color32::from_rgb(r, g, b)
            }

            Self::QuantumPlasma => {
                // High-energy plasma with quantum fluctuations
                let energy = smoothed.powf(0.5);
                let quantum_flux = (energy * 12.0 * PI).sin();
                let plasma_wave = (energy * 8.0 * PI + 2.0).sin();
                let field_strength = (energy * 15.0 * PI).cos().abs();
                let instability = (energy * 25.0 * PI).sin() * 0.1 + 0.9;

                // Electric blue, violet, and white
                let electric = (0.5 + 0.5 * quantum_flux) * instability;
                let violet = (0.5 + 0.5 * plasma_wave) * field_strength;
                let intensity = energy.sqrt();

                let r = (100.0 + 155.0 * violet * intensity) as u8;
                let g = (80.0 + 100.0 * electric * intensity) as u8;
                let b = (200.0 + 55.0 * (electric + violet) * 0.5) as u8;
                Color32::from_rgb(r, g, b)
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
