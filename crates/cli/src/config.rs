use std::{env::Args, path::PathBuf};

use kankinge_solver_rs_core::{Algorithm, Config};

pub(crate) fn parse(mut args: Args) -> Result<(PathBuf, Config), &'static str> {
    args.next();

    let stage_path: PathBuf = args
        .next()
        .map(PathBuf::from)
        .ok_or("USAGE: ksr <stage_file> <bfs|iddfs>")?;

    let solve_config = args
        .next()
        .ok_or("USAGE: ksr <stage_file> <bfs|iddfs>")?
        .parse()
        .map(|algorithm| match algorithm {
            Algorithm::Bfs => Config::Bfs { skip_drop: true },
            Algorithm::Iddfs => Config::Iddfs,
        })
        .map_err(|_| "USAGE: ksr <stage_file> <bfs|iddfs>")?;

    Ok((stage_path, solve_config))
}
