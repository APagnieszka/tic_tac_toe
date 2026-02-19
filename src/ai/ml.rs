use crate::board::Board;
use crate::cell::Cell;

use super::AiStrategy;

pub struct MlAi;

impl AiStrategy for MlAi {
    fn choose_move(&self, board: &Board) -> Option<usize> {
        for i in 0..9 {
            if board.fields[i] == Cell::Empty {
                return Some(i);
            }
        }
        None
    }
}