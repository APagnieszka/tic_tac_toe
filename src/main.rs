
mod board;
mod cell;

use board::Board;

fn main() {
    println!("Hello, world!");

    let mut board = Board::new();

    board.display();

    let success = board.make_move(0, cell::Cell::X);

    board.display();
}



