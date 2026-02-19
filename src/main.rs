
mod board;
mod cell;

use board::Board;

fn main() {
    println!("Hello, world!");

    let board = Board::new();

    Board::print(&board);
}



