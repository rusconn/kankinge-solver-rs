mod algorithms;
mod object;
mod stage;
mod state;

use strum::EnumString;

use crate::{
    algorithms::{bfs, iddfs},
    stage::InstanceId,
};

pub use stage::Stage;
pub use state::Status;

#[must_use]
pub fn solve(stage: &Stage, config: Config) -> Option<Solution> {
    match config {
        Config::Bfs { skip_drop } => bfs::run(stage, skip_drop),
        Config::Iddfs => iddfs::run(stage),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Config {
    Bfs { skip_drop: bool },
    Iddfs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Algorithm {
    Bfs,
    Iddfs,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Solution {
    pub status: Status,
    pub steps: Vec<&'static str>,
}

impl Solution {
    pub(crate) fn new(status: Status, moved_tos: &[InstanceId], stage: &Stage) -> Self {
        let mut steps = Vec::with_capacity(moved_tos.len());

        for &moved_to in moved_tos {
            let instance = &stage.instance_of(moved_to);
            steps.push(instance.name());
        }

        Self { status, steps }
    }
}
