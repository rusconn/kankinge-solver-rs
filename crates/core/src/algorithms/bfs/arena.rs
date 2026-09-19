use std::ops::Index;

use super::Node;

pub(crate) struct Arena(Vec<Node>);

impl Arena {
    pub(crate) fn next_index(&self) -> ArenaIndex {
        ArenaIndex(self.0.len())
    }

    pub(crate) fn push(&mut self, node: Node) {
        self.0.push(node);
    }

    pub(crate) fn pop(&mut self) -> Option<Node> {
        self.0.pop()
    }
}

impl<const N: usize> From<[Node; N]> for Arena {
    fn from(value: [Node; N]) -> Self {
        Self(Vec::from(value))
    }
}

impl Index<ArenaIndex> for Arena {
    type Output = Node;

    fn index(&self, index: ArenaIndex) -> &Self::Output {
        &self.0[index.0]
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArenaIndex(usize);

impl ArenaIndex {
    pub(crate) const FIRST: Self = Self(0);
}
