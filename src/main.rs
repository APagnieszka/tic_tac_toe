mod board;
mod cell;

use board::Board;
use std::io;

fn main() {

    let mut board = Board::new();

    board.display();

    loop {
        let player_move = get_player_move();
        if !board.make_move(player_move, cell::Cell::O) {
            println!("Invalid move, cell might be occupied.");
            continue;
        }
        board.display();

        if board.check_winner().is_some() || board.is_full() {
            break;
        }

        if let Some(best_move) = board.find_best_move() {
            board.make_move(best_move, cell::Cell::X);
        }
        board.display();

        if board.check_winner().is_some() || board.is_full() {
            break;
        }
    }

    board.display_winner();
}

fn get_player_move() -> usize {
    loop {
        println!("Enter your move (0-8): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) if num < 9 => return num,
            _ => println!("Invalid input. Please enter a number between 0 and 8."),
        }
    }
}
