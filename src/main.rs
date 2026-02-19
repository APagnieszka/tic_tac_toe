mod board;
mod cell;
mod game_cli;
mod game_window;

use std::env;

use game_cli::GameCli;
use game_window::GameWindow;

enum RunMode {
    Cli,
    Window,
}

fn run_mode_from_args() -> RunMode {
    match env::args().nth(1).as_deref() {
        Some("cli") => RunMode::Cli,
        _ => RunMode::Window,
    }
}

#[macroquad::main("Tic Tac Toe")]
async fn main() {
    match run_mode_from_args() {
        RunMode::Cli => {
            let mut game = GameCli::new();
            game.run();
        }
        RunMode::Window => {
            let mut game = GameWindow::new();
            game.run().await;
        }
    }
}
