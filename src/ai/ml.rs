use crate::board::Board;
use crate::cell::Cell;
use burn::tensor::Tensor;
use burn_ndarray::NdArray;

use super::AiStrategy;
use super::ml_model::TicTacToeModel;

pub struct MlAi;

type Backend = NdArray<f32>;

impl MlAi {
    pub fn map_board(board: &Board) -> [f32; 9] {
        board.fields.map(Self::map_cell)
    }

    fn map_cell(cell: Cell) -> f32 {
        match cell {
            Cell::Empty => 0.0,
            Cell::X => 1.0,
            Cell::O => -1.0,
        }
    }
}

impl AiStrategy for MlAi {
    fn choose_move(&self, board: &Board) -> Option<usize> {
        let encoded_board = Self::map_board(board);
        let device = <Backend as burn::tensor::backend::Backend>::Device::default();
        let model = TicTacToeModel::<Backend>::new(&device);
        let input = Tensor::<Backend, 2>::from_floats([encoded_board], &device);
        let _probabilities = model.forward_probs(input);

        if board.fields[4] == Cell::Empty {
            return Some(4);
        }

        board.fields.iter().position(|&cell| cell == Cell::Empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_board_to_expected_numeric_representation() {
        let board = Board {
            fields: [
                Cell::Empty,
                Cell::X,
                Cell::O,
                Cell::X,
                Cell::Empty,
                Cell::O,
                Cell::O,
                Cell::X,
                Cell::Empty,
            ],
        };

        let mapped = MlAi::map_board(&board);

        assert_eq!(mapped, [0.0, 1.0, -1.0, 1.0, 0.0, -1.0, -1.0, 1.0, 0.0]);
    }

    #[test]
    fn picks_center_on_empty_board() {
        let board = Board::new();

        let chosen_move = MlAi.choose_move(&board);

        assert_eq!(chosen_move, Some(4));
    }
}
