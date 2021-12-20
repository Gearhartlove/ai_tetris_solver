use std::fmt::{Formatter};
//use crate::board::cell::Color::Unassigned;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Copy, Clone)]
pub struct Cell {
    pub x_pos: usize,
    pub y_pos: usize,
    pub id: Color,
    pub block: bool,
}

impl Cell {
    pub fn new(x: usize, y:usize) -> Self{
        Self { x_pos: x, y_pos: y, id: rand::thread_rng().gen(), block: true}
    }

    fn get_id(&self) -> &str {
        match self.id {
            Color::Green => "G",
            Color::Blue => "B",
            Color::Purple => "P",
            Color::Red => "R",
            Color::Yellow => "Y",
            _ => "X",
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.get_id())
    }
}

/// Enum for printout out the shapes to the terminal
#[derive(Copy, Clone)]
pub enum Color {
    Unassigned,
    Blue,
    Red,
    Purple,
    Green,
    Yellow,
}

impl Distribution<Color> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color{
        match rng.gen_range(0..=5) {
            0 => Color::Blue,
            1 => Color::Red,
            2 => Color::Purple,
            3 => Color::Green,
            _ => Color::Yellow,
        }
    }
}