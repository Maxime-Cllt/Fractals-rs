pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    
    /// Creates a new `Point` with the given x and y coordinates.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
