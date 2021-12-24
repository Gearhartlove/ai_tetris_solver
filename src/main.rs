mod board;

fn main() {
    let mut board = board::Board::new(10, 20);
    // board.place_block(2, 0, Square{});
    println!("{}", board);
}
