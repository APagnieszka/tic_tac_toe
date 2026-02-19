use tic_tac_toe::ai::AiMode;
use tic_tac_toe::args::{RunMode, ai_mode_from_args, run_mode_from_args};

#[test]
fn parses_cli_mode_and_ml_ai_from_public_api() {
    let args = vec!["app".to_string(), "cli".to_string(), "ml".to_string()];

    assert!(matches!(run_mode_from_args(&args), RunMode::Cli));
    assert!(matches!(ai_mode_from_args(&args), AiMode::Ml));
}

#[test]
fn defaults_to_window_and_minimax_from_public_api() {
    let args = vec!["app".to_string()];

    assert!(matches!(run_mode_from_args(&args), RunMode::Window));
    assert!(matches!(ai_mode_from_args(&args), AiMode::Minimax));
}
