use std::io;

use crate::ai::{AiMode, AiStrategy, build_ai};
use crate::board::Board;
use crate::cell::Cell;

pub struct GameCli {
    board: Board,
    ai: Box<dyn AiStrategy>,
}

impl GameCli {
    pub fn new(ai_mode: AiMode) -> Self {
        Self {
            board: Board::new(),
            ai: build_ai(ai_mode),
        }
    }

    pub fn run(&mut self) {
        self.board.display();

        loop {
            let player_move = Self::get_player_move();
            if !self.board.make_move(player_move, Cell::O) {
                println!("Invalid move, cell might be occupied.");
                continue;
            }
            self.board.display();

            if self.board.check_winner().is_some() || self.board.is_full() {
                break;
            }

            if let Some(best_move) = self.ai.choose_move(&self.board) {
                self.board.make_move(best_move, Cell::X);
            }
            self.board.display();

            if self.board.check_winner().is_some() || self.board.is_full() {
                break;
            }
        }

        self.board.display_winner();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_cli_new_accepts_ai_mode() {
        let _game = GameCli::new(AiMode::Minimax);
    }
}
