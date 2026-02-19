use burn::module::Module;
use burn::nn::{Linear, LinearConfig};
use burn::tensor::Tensor;
use burn::tensor::activation::{relu, softmax};
use burn::tensor::backend::Backend;

/// Simple MLP planned for Tic-Tac-Toe move scoring.
///
/// ReLU (Rectified Linear Unit) is an activation function defined as:
/// `ReLU(x) = max(0, x)`. It keeps positive values and sets negatives to 0,
/// adding non-linearity while staying fast and stable in practice.
#[derive(Module, Debug)]
pub struct TicTacToeModel<B: Backend> {
    /// Input layer: projects board features (9 cells) into hidden space.
    input: Linear<B>,
    /// Hidden layer: learns intermediate feature interactions.
    hidden: Linear<B>,
    /// Output layer: produces one score per board position (9 logits).
    output: Linear<B>,
}

impl<B: Backend> TicTacToeModel<B> {
    pub fn new(device: &B::Device) -> Self {
        Self {
            input: LinearConfig::new(9, 64).init(device),
            hidden: LinearConfig::new(64, 64).init(device),
            output: LinearConfig::new(64, 9).init(device),
        }
    }

    // input -> ReLU(hidden) -> ReLU(hidden) -> output logits.
    pub fn forward_logits(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let hidden = self.input.forward(input);
        let hidden = relu(hidden);
        let hidden = self.hidden.forward(hidden);
        let hidden = relu(hidden);

        self.output.forward(hidden)
    }

    // Converts logits to probabilities over the 9 board positions.
    pub fn forward_probs(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let logits = self.forward_logits(input);
        softmax(logits, 1)
    }
}
