use std::rc::Rc;

use crate::{
    stage::{InstanceId, Stage},
    state::{Child, State},
};

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) depth: u16,
    pub(crate) moved_to: Option<InstanceId>,
    pub(crate) state: State,
    pub(crate) parent: Option<Rc<Self>>,
}

impl Node {
    pub(crate) fn root(stage: &Stage) -> Self {
        Self {
            depth: 0,
            moved_to: None,
            state: State::initial(stage),
            parent: None,
        }
    }

    pub(crate) fn child(parent: &Rc<Self>, child: Child) -> Self {
        Self {
            depth: parent.depth + 1,
            moved_to: child.moved_to,
            state: child.state,
            parent: Some(Rc::clone(parent)),
        }
    }
}
