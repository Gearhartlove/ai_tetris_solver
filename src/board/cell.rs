use std::fmt::{Formatter};

use crate::board::block::Shape;
use crate::board::block::Color;
use crate::board::BlockType;

//#[derive(Copy, Clone)]
pub struct Cell {
    pub x_pos: i8,
    pub y_pos: i8,
    // pub id: Color,
    pub block: Option<&'static dyn Shape>,
}

impl Cell {
    pub fn new(x: i8, y: i8) -> Self{
        Self { x_pos: x, y_pos: y, block: None}
    }

    // gets the color of the block for the print out
    fn get_id(&self) -> &str {
        match self.block {
            None => "0",
            Some(block) => match block.get_color() {
                Color::Green => "G",
                Color::Blue => "B",
                Color::Purple => "P",
                Color::Red => "R",
                Color::Yellow => "Y",
                _ => "X",
            }
        // match self.block {
            // Color::Green => "G",
            // Color::Blue => "B",
            // Color::Purple => "P",
            // Color::Red => "R",
            // Color::Yellow => "Y",
            // _ => "X",
        }
    }

    pub fn set_cell_true(&mut self, block: &'static dyn Shape) {
        self.block = Some(block);
    }

    // pub fn set_cell_false(&mut self) {
    //     self.block = false
    // }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.get_id())
    }
}

// Enum for printout out the shapes to the terminal
// #[derive(Copy, Clone)]
// pub enum Color {
//     Unassigned,
//     Blue,
//     Red,
//     Purple,
//     Green,
//     Yellow,
// }
