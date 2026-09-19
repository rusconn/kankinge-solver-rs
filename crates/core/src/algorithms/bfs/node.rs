use std::cell::Cell;

use crate::{
    stage::Stage,
    state::{Action, Child, State},
};

use super::ArenaIndex;

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) depth: u16,
    pub(crate) action: Action,
    pub(crate) state: State,
    pub(crate) parent: Option<ArenaIndex>,
    pub(crate) alive: Cell<bool>,
}

impl Node {
    pub(crate) fn root(stage: &Stage) -> Self {
        Self {
            depth: 0,
            action: Action::Root,
            state: State::initial(stage),
            parent: None,
            alive: Cell::new(true),
        }
    }

    pub(crate) fn child(parent_idx: ArenaIndex, parent_depth: u16, child: Child) -> Self {
        Self {
            depth: parent_depth + 1,
            action: child.action,
            state: child.state,
            parent: Some(parent_idx),
            alive: Cell::new(true),
        }
    }
}
