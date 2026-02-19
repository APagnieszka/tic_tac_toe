use crate::board::Board;
use crate::cell::Cell;

use super::AiStrategy;

pub struct MlAi;

impl AiStrategy for MlAi {
    fn choose_move(&self, board: &Board) -> Option<usize> {
        (0..9).find(|&i| board.fields[i] == Cell::Empty)
    }
}
