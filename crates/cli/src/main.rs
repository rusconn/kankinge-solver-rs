mod config;

use std::{env, fs, process::ExitCode};

use kankinge_solver_rs_core::{Stage, solve};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> ExitCode {
    let (stage_path, solve_config) = match config::parse(env::args()) {
        Ok(v) => v,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::FAILURE;
        }
    };

    let stage_json = match fs::read_to_string(&stage_path) {
        Ok(stage_json) => stage_json,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let stage = match Stage::parse(&stage_json) {
        Ok(stage) => stage,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    let solution = solve(&stage, solve_config);

    println!(
        "{}",
        solution.map_or(
            r#""impossible""#.into(), //
            |solution| serde_json::to_string(&solution).unwrap(),
        )
    );

    ExitCode::SUCCESS
}
