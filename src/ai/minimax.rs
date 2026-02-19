use crate::board::Board;
use crate::cell::Cell;

use super::AiStrategy;

pub struct MinimaxAi;

impl MinimaxAi {
    fn minimax(&self, board: &mut Board, depth: usize, is_maximizing: bool) -> i32 {
        if let Some(winner) = board.check_winner() {
            return match winner {
                Cell::X => 10 + depth as i32,
                Cell::O => -10 - depth as i32,
                Cell::Empty => 0,
            };
        }

        if board.is_full() {
            return 0;
        }

        if is_maximizing {
            let mut best_score = i32::MIN;
            for i in 0..9 {
                if board.fields[i] == Cell::Empty {
                    board.fields[i] = Cell::X;
                    let score = self.minimax(board, depth + 1, false);
                    board.fields[i] = Cell::Empty;
                    best_score = best_score.max(score);
                }
            }
            best_score
        } else {
            let mut best_score = i32::MAX;
            for i in 0..9 {
                if board.fields[i] == Cell::Empty {
                    board.fields[i] = Cell::O;
                    let score = self.minimax(board, depth + 1, true);
                    board.fields[i] = Cell::Empty;
                    best_score = best_score.min(score);
                }
            }
            best_score
        }
    }
}

impl AiStrategy for MinimaxAi {
    fn choose_move(&self, board: &Board) -> Option<usize> {
        let mut board = *board;
        let mut best_score = i32::MIN;
        let mut best_move = None;

        for i in 0..9 {
            if board.fields[i] == Cell::Empty {
                board.fields[i] = Cell::X;
                let score = self.minimax(&mut board, 0, false);
                board.fields[i] = Cell::Empty;

                if score > best_score {
                    best_score = score;
                    best_move = Some(i);
                }
            }
        }

        best_move
    }
}
