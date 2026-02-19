pub mod minimax;
pub mod ml;
mod ml_model;

use minimax::MinimaxAi;
use ml::MlAi;

use crate::board::Board;

#[derive(Clone, Copy)]
pub enum AiMode {
    Minimax,
    Ml,
}

pub trait AiStrategy {
    fn choose_move(&self, board: &Board) -> Option<usize>;
}

pub fn build_ai(mode: AiMode) -> Box<dyn AiStrategy> {
    match mode {
        AiMode::Minimax => Box::new(MinimaxAi),
        AiMode::Ml => Box::new(MlAi),
    }
}
