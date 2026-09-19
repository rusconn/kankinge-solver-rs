use std::rc::Rc;

use crate::{
    stage::Stage,
    state::{Action, Child, State},
};

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) depth: u16,
    pub(crate) action: Action,
    pub(crate) state: State,
    pub(crate) parent: Option<Rc<Self>>,
}

impl Node {
    pub(crate) fn root(stage: &Stage) -> Self {
        Self {
            depth: 0,
            action: Action::Root,
            state: State::initial(stage),
            parent: None,
        }
    }

    pub(crate) fn child(parent: &Rc<Self>, child: Child) -> Self {
        Self {
            depth: parent.depth + 1,
            action: child.action,
            state: child.state,
            parent: Some(Rc::clone(parent)),
        }
    }
}
