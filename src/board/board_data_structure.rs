use std::fmt::Formatter;
use crate::board::block::{block_factory, BlockType, Shape};
use crate::board::cell::Cell;

pub struct Board {
    height_const: i8,
    pub width_const: i8,
    cells: Vec<Cell>,
    play_blocks: Vec<&'static dyn Shape>, //blocks included in the tetris game // dyn?
}

/// Foundational data structure to run Tetris
impl Board {
    /// Constructor for Board Struct
    pub fn new(width: usize, height: usize) -> Self {
        let mut temp_cells: Vec<Cell> = Vec::new();
        // add to allowed tetris blocks
        let blocks: Vec<&dyn Shape> = vec![
            // block_factory(BlockType::T),
            block_factory(BlockType::Square)];

        let mut y: usize = 0;
        let mut x: usize = 0;
        // Loop function: instantiate x and y perms for the board
        loop {
            // create a new cell and add it to the vector
            temp_cells.push(Cell::new(x as i8,y as i8));
            // When the vector is the desired length, exit the loop
            if temp_cells.len() >= height * width {
                break;
            }
            x += 1;
            if x % width == 0 {
                y += 1;
                x = 0;
            }
        }
        //Self { WIDTH: width, HEIGHT: height, CELLS: cells, BLOCKS: blocks}
        Self {width_const: width as i8, height_const: height as i8, cells: temp_cells, play_blocks: blocks}
    }

    // check if out of bounds
    fn is_oob(&self, x: usize, y:usize, i:usize, j:usize) -> bool {
        return if x + i >= self.width_const as usize || y + j >= self.width_const as usize {
            true
        } else {
            return false;
        }
    }

    //TODO: consider parameters, what's needed?
    fn block_check(&self, x: usize, y: usize, block: &'static dyn Shape) -> bool {
        for mut add_y in 0..block.get_size() {
            for mut add_x in 0..block.get_size() {
                if !self.is_oob(x,y, add_x, add_y) {

                }
            }
        }
        return true;
    }

    // IDEA: write tests seeing if I can place different blocks in different positions
    // note: this should be able to place the different rotations of blocks
    // NOTE: out of bounds is not fixed by this issue ... need to address OOB issues too
    pub fn place_block(&self, x: usize, y: usize, block: &'static dyn Shape) {
        // check if block is placeable; if true statement
        // for mut add_y in 0..block.get_size() {
        //     for mut add_x in 0..block.get_size() {
        //         // don't consider if query is out of bounds
        //         if !is_oob(x, y, add_x, add_y) {
        //             match self.cells[self.index(x + add_x, y + add_y)].block {
        //                 None => ,
        //                 Some(block) => panic!("Cannot place block,")
        //             }
        //             // x += 1;
        //         }
        //         n += 1;
        //     }
        // }

        //place the block on the board
        // match self.cells[self.index(x,y)].block {
        //     // don't allow block to be placed on the board if there is already a block there
        //     None => println!("Can't place block at {} {}, block already present", x, y),
        //     // place the block onto the board
        //     //Q: can I change the board because it is not &mut self?
        //     Some(block) => {
        //
        //     },
        // }
    }

    // Note: need to make a cell getter eventually
    // method to set a cell to true
    // fn set_cell_block(&mut self, x: usize, y: usize, block: &'static dyn Shape) {
    //     self.cells[index(x,y, self.width_const as usize)].block = Some(block);
    // }

    // // method to set a cell to true
    // pub fn set_cell_false(&mut self, x: usize, y: usize) {
    //     self.cells_const[x + y * self.width_const as usize].block = false;
    // }

    // gets an index of the board (cleaner syntax function)

    pub fn get_cell(&self, x: usize, y: usize, width: usize) -> Result<&Cell, String>{
        match self.cells.get(x + y * width) {
            Some(cell) => Ok(cell),
            None => Err(String::from("the board index does not exist")), // add coords
        }
    }
}



/// Pretty printing to the Console
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("");
        // assign cells to correct x, y positions on the board
        for cell in &self.cells {
            match cell.block {
                Some(block) => {
                        // TODO: determine why this is returning cell instead of &str
                        output.push_str(cell.to_string().trim());
                        output.push_str("  ")
                }
                None => output.push_str("0  ")
            }


            if &self.width_const-1 == cell.x_pos {
                output.push_str("\n");
            }
        }

        write!{f, "{}", output}
    }
}
