use crate::ai::AiMode;

pub enum RunMode {
    Cli,
    Window,
}

pub fn run_mode_from_args(args: &[String]) -> RunMode {
    match args.get(1).map(String::as_str) {
        Some("cli") => RunMode::Cli,
        _ => RunMode::Window,
    }
}

pub fn ai_mode_from_args(args: &[String]) -> AiMode {
    let ai_arg = if matches!(args.get(1).map(String::as_str), Some("cli" | "window")) {
        args.get(2)
    } else {
        args.get(1)
    };

    match ai_arg.map(String::as_str) {
        Some("ml") => AiMode::Ml,
        _ => AiMode::Minimax,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_mode_is_cli_for_cli_arg() {
        let args = vec!["app".to_string(), "cli".to_string()];
        assert!(matches!(run_mode_from_args(&args), RunMode::Cli));
    }

    #[test]
    fn run_mode_defaults_to_window_without_cli_arg() {
        let args = vec!["app".to_string()];
        assert!(matches!(run_mode_from_args(&args), RunMode::Window));
    }

    #[test]
    fn ai_mode_is_ml_when_ml_arg_is_passed() {
        let args = vec!["app".to_string(), "cli".to_string(), "ml".to_string()];
        assert!(matches!(ai_mode_from_args(&args), AiMode::Ml));
    }

    #[test]
    fn ai_mode_defaults_to_minimax() {
        let args = vec!["app".to_string(), "cli".to_string()];
        assert!(matches!(ai_mode_from_args(&args), AiMode::Minimax));
    }
}
