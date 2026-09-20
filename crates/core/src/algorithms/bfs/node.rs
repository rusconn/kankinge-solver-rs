use std::cell::Cell;

use crate::{
    stage::{InstanceId, Stage},
    state::{Child, State},
};

use super::ArenaIndex;

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) depth: u16,
    pub(crate) moved_to: Option<InstanceId>,
    pub(crate) state: State,
    pub(crate) parent: Option<ArenaIndex>,
    pub(crate) alive: Cell<bool>,
}

impl Node {
    pub(crate) fn root(stage: &Stage) -> Self {
        Self {
            depth: 0,
            moved_to: None,
            state: State::initial(stage),
            parent: None,
            alive: Cell::new(true),
        }
    }

    pub(crate) fn child(parent_idx: ArenaIndex, parent_depth: u16, child: Child) -> Self {
        Self {
            depth: parent_depth + 1,
            moved_to: child.moved_to,
            state: child.state,
            parent: Some(parent_idx),
            alive: Cell::new(true),
        }
    }
}
