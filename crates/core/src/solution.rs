use crate::{
    stage::{InstanceId, Stage},
    state::Status,
};

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
