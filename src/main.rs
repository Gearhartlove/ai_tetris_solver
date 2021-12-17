mod board;

fn main() {
    let board = board::Board::new(10, 20);
    println!("{}", board)
}
