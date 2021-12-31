use crate::board::cell::Cell;
//use crate::board::block::block_type;

// Places a block onto the Tetris board.
pub fn place_block(cell: &mut Cell) {
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
    fn place_simple_block() {
        let mut board = Board::new(1,1);
        let cell_to_change = board.get_cell(0,0).unwrap();
        place_block(cell_to_change);

        assert_eq!("1  \n", board.to_string());
    }

    #[test]
    fn place_multiple_blocks() {
        let mut board = Board::new(3,3);
        let cell_to_change = board.get_cell(1, 1).unwrap();
        place_block(cell_to_change);
        let cell_to_change_2 = board.get_cell(2, 1).unwrap();
        place_block(cell_to_change_2);

        assert_eq!(
            "0  0  0  \n0  1  1  \n0  0  0  \n", board.to_string()
        )
    }
}