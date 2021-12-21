mod board;

fn main() {
    let mut board = board::Board::new(10, 20);
    // block type, position (x,y)
    // board.cell(2,0).set_cell_true();
    // board.set_cell_true(2,0);
    // board.set_cell_true(2,1);
    // board.set_cell_true(3,0);
    // board.set_cell_true(3,1);
    println!("{}", board);
}
