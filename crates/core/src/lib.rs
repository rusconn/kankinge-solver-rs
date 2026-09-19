mod algorithms;
mod object;
mod point;
mod stage;
mod state;

use strum::EnumString;

use crate::{
    algorithms::{bfs, iddfs},
    state::Action,
};

pub use point::Point;
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
    pub steps: Vec<Step>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "action", rename_all = "snake_case"))]
pub enum Step {
    Move { name: &'static str, point: Point },
    Convert { kind: ConvertKind },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ConvertKind {
    Silver,
    Gold,
    Hp,
    Atk,
    Def,
}

impl Solution {
    pub(crate) fn new(status: Status, actions: &[Action], stage: &Stage) -> Self {
        let mut steps = Vec::with_capacity(actions.len());

        for action in actions {
            match action {
                Action::Root => {}
                Action::Move(dest) => {
                    let instance = stage.instance_of(*dest);
                    steps.push(Step::Move {
                        name: instance.name(),
                        point: stage.point_of(instance),
                    });
                }
                &Action::Convert(kind) => {
                    steps.push(Step::Convert { kind });
                }
            }
        }

        Self { status, steps }
    }
}
