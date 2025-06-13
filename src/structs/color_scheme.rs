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
        }
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
            ColorScheme::Classic => {
                let r = (255.0 * (0.5 + 0.5 * (4.0 * smoothed).sin())) as u8;
                let g = (255.0 * (0.5 + 0.5 * (2.0 * smoothed + 2.0).sin())) as u8;
                let b = (255.0 * (1.0 - smoothed).powf(0.3)) as u8;
                Color32::from_rgb(r, g, b)
            }
            ColorScheme::Hot => {
                if smoothed < 0.25 {
                    let t = smoothed * 4.0;
                    Color32::from_rgb((80.0 + 175.0 * t) as u8, (20.0 * t) as u8, 0)
                } else if smoothed < 0.5 {
                    let t = (smoothed - 0.25) * 4.0;
                    Color32::from_rgb(255, (20.0 + 235.0 * t) as u8, 0)
                } else if smoothed < 0.75 {
                    let t = (smoothed - 0.5) * 4.0;
                    Color32::from_rgb(255, 255, (200.0 * t) as u8)
                } else {
                    let t = (smoothed - 0.75) * 4.0;
                    Color32::from_rgb(255, 255, (200.0 + 55.0 * t) as u8)
                }
            }
            ColorScheme::Cool => {
                // Ocean-inspired cool palette
                let r = (100.0 * (1.0 - smoothed).powf(2.0)) as u8;
                let g = (50.0 + 205.0 * smoothed.powf(0.7)) as u8;
                let b = (150.0 + 105.0 * smoothed) as u8;
                Color32::from_rgb(r, g, b)
            }
            ColorScheme::Psychedelic => {
                let angle = smoothed * 6.28318 * 3.0;
                let r = (127.5 + 127.5 * angle.sin()) as u8;
                let g = (127.5 + 127.5 * (angle + 2.094).sin()) as u8; 
                let b = (127.5 + 127.5 * (angle + 4.188).sin()) as u8;
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
    }
}
