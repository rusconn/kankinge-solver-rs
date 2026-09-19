use crate::{
    point::Point,
    stage::Stage,
    state::{Action, ConvertKind, Status},
};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Solution {
    pub status: Status,
    pub steps: Vec<Step>,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(tag = "action", rename_all = "snake_case")
)]
pub enum Step {
    Move { name: &'static str, point: Point },
    Convert { kind: ConvertKind },
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
