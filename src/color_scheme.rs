#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorScheme {
    Classic,
    Hot,
    Cool,
    Grayscale,
}

impl ColorScheme {
    pub fn name(&self) -> &'static str {
        match self {
            ColorScheme::Classic => "Classic",
            ColorScheme::Hot => "Hot",
            ColorScheme::Cool => "Cool",
            ColorScheme::Grayscale => "Grayscale",
        }
    }
}