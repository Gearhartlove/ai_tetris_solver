use crate::board::cell::Cell;
//use crate::board::block::block_type;

// Places a block onto the Tetris board.
pub fn place_block(cell: &Cell) {
    cell.set_cell_true();
}

// fn rotate_block(board: &Board, block: block_type) -> Vec<Vec<usize>> {
//
// }

// Unit Tests
#[cfg(test)]
mod tests {
    use crate::Board;
    use super::*;

    #[test]
    fn place_simple_block_success() {
        let board = Board::new(3,3);
        place_block(board.get_cell(1, 1, board.width_const as usize).unwrap());
        assert_eq!(
            "0  0  0  \n0  1  0  \n0  0  0  \n", board.to_string()

        )
    }
}