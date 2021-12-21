use std::borrow::Borrow;
use std::fmt::Formatter;
 use crate::board::block::{block_factory, BlockType, Shape};
use crate::board::cell::Cell;

pub struct Board {
    height_const: i8,
    width_const: i8,
    cells_const: Vec<Cell>,
    play_blocks: Vec<&'static dyn Shape>, //blocks included in the tetris game // dyn?
}

/// Foundational data structure to run Tetris
impl Board {
    /// Constructor for Board Struct
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        let blocks: Vec<&dyn Shape> = vec![
            block_factory(BlockType::T),
            block_factory(BlockType::Square)];

        let mut y: usize = 0;
        let mut x: usize = 0;
        // Loop function: instantiate x and y perms for the board
        loop {
            // create a new cell and add it to the vector
            cells.push(Cell::new(x as i8,y as i8));
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
        //Self { WIDTH: width, HEIGHT: height, CELLS: cells, BLOCKS: blocks}
        Self {width_const: width as i8, height_const: height as i8, cells_const: cells, play_blocks: blocks}
    }

    // Note: need to make a cell getter eventually
    // method to set a cell to true
    pub fn set_cell_true(&mut self, x: usize, y: usize) {
        self.cells_const[x + y * self.width_const as usize].block = true;
    }

    // method to set a cell to true
    pub fn set_cell_false(&mut self, x: usize, y: usize) {
        self.cells_const[x + y * self.width_const as usize].block = false;
    }
}

/// Pretty printing to the Console
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("");
        // assign cells to correct x, y positions on the board
        for cell in &self.cells_const {
            if cell.block == true {
                // TODO: determine why this is returning cell instead of &str
                output.push_str(cell.to_string().trim());
                output.push_str("  ")
            } else {
                output.push_str("0  ");
            }


            if &self.width_const-1 == cell.x_pos {
                output.push_str("\n");
            }
        }

        write!{f, "{}", output}
    }
}
