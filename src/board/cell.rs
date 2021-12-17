use std::fmt::{Formatter, write};

#[derive(Copy, Clone)]
pub struct Cell {
    pub X_POS: usize,
    pub Y_POS: usize,
}

impl Cell {
    pub fn new(x: usize, y:usize) -> Self{
        Self { X_POS: x, Y_POS: y}
    }
}