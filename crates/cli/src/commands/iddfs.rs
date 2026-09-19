use std::process::ExitCode;

use kankinge_solver_rs_core::{Stage, iddfs};

pub(crate) fn solve(stage: &Stage) -> ExitCode {
    let Some(solution) = iddfs(stage) else {
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
