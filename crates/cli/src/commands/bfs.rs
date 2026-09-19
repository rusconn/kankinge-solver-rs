use std::process::ExitCode;

use kankinge_solver_rs_core::{Stage, bfs};

#[derive(Debug, Clone, Copy, clap::Args)]
pub(crate) struct Args {
    /// Allows solver to leak memory. Reduces the time taken to free memory.
    #[arg(long)]
    allow_leak: bool,
}

pub(crate) fn solve(stage: &Stage, args: Args) -> ExitCode {
    let Some(solution) = bfs(stage, args.allow_leak) else {
        println!(r#""impossible""#);
        return ExitCode::SUCCESS;
    };

    match serde_json::to_string(&solution) {
        Ok(json) => {
            println!("{json}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("failed to serialize solution: {e}");
            ExitCode::FAILURE
        }
    }
}
