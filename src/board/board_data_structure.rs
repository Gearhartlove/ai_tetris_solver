use std::fmt::Formatter;
use crate::board::cell::Cell;

pub struct Board {
    HEIGHT: usize,
    WIDTH: usize,
    CELLS: Vec<Cell>,
}

/// Foundational data structure to run Tetris

impl Board {
    /// Constructor for Board Struct
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Cell> = Vec::new();

        let mut y = 0;
        let mut x = 0;
        // Loop function: instantiate x and y perms for the board
        loop {
            // create a new cell and add it to the vector
            cells.push(Cell::new(x,y));
            // When the vector is the desired length, exit the loop
            if cells.len() >= height * width {
                break;
            }
            x += 1;
            if x % width == 0 {
                y += 1;
                x = 0;
            }
        }
        Self { WIDTH: width, HEIGHT: height, CELLS: cells}
    }
}

/// Pretty printing to the Console
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("");
        for cell in &self.CELLS {
            output.push_str("0 ");

            if &self.WIDTH-1 == cell.X_POS {
                output.push_str("\n");
            }
        }
        write!{f, "{}", output}
    }
}