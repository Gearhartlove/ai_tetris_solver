use std::fmt::{Formatter};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use crate::board::block::Shape;
use crate::board::BlockType;

//#[derive(Copy, Clone)]
#[derive(Clone)]
pub struct Cell {
    pub x_pos: i8,
    pub y_pos: i8,
    pub id: Color,
    pub block: Option<&'static dyn Shape>,
}

impl Cell {
    pub fn new(x: i8, y: i8) -> Self{
        Self { x_pos: x, y_pos: y, id: rand::thread_rng().gen(), block: None}
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

    // pub fn set_cell_true(&mut self) {
    //     self.block = true
    // }
    //
    // pub fn set_cell_false(&mut self) {
    //     self.block = false
    // }
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