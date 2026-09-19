mod args;
mod commands;

use std::{fs, process::ExitCode};

use clap::Parser;
use kankinge_solver_rs_core::Stage;

use args::{Args, Command};
use commands::{bfs, iddfs};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> ExitCode {
    let args = Args::parse();

    let stage_json = match fs::read_to_string(&args.stage_json) {
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

    match &args.command {
        Command::Bfs(args) => bfs::solve(&stage, *args),
        Command::Iddfs => iddfs::solve(&stage),
    }
}
