use crate::board::Board;
use crate::board::BlockType;
use crate::board::cell::Cell;
//use crate::board::block::block_type;

/// Places a block onto the Tetris board.
pub fn place_block(cell: &mut Cell) {
    // &board.cell(x,y).block_status(true);
    cell.set_cell_true();
}

// fn rotate_block(board: &Board, block: block_type) -> Vec<Vec<usize>> {
//
// }