mod board;
mod cell;
mod game_cli;

use game_cli::GameCli;

fn main() {
    let mut game = GameCli::new();
    game.run();
}
