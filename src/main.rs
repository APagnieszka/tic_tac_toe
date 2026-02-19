use std::env;

use tic_tac_toe::args::{RunMode, ai_mode_from_args, run_mode_from_args};
use tic_tac_toe::game_cli::GameCli;
use tic_tac_toe::game_window::GameWindow;

#[macroquad::main("Tic Tac Toe")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let run_mode = run_mode_from_args(&args);
    let ai_mode = ai_mode_from_args(&args);

    match run_mode {
        RunMode::Cli => {
            let mut game = GameCli::new(ai_mode);
            game.run();
        }
        RunMode::Window => {
            let mut game = GameWindow::new(ai_mode);
            game.run().await;
        }
    }
}

