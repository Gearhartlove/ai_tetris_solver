
pub struct Board {
    HEIGHT: usize,
    WIDTH: usize,
}

impl Board {
    /// Constructor for Board Struct
    pub fn new(height: usize, width: usize) -> Self {
        Self { HEIGHT: height, WIDTH: width}
    }
}