mod board;
mod cell;

use board::Board;

fn main() {
    println!("Hello, world!");

    let mut board = Board::new();

    board.display();

    let success = board.make_move(0, cell::Cell::X);

    board.display();

    board.display_winner();
    let success = board.make_move(1, cell::Cell::O);
    let success = board.make_move(2, cell::Cell::X);
    board.display_winner();
}
