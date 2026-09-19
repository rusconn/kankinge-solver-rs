mod node;

use std::{rc::Rc, time::Instant};

use crate::{Solution, stage::Stage};

use node::Node;

pub(crate) fn run(stage: &Stage) -> Option<Solution> {
    let mut prev_searched = 0;

    for limit in 0.. {
        let begin = Instant::now();
        let (searched, goal) = dls(stage, limit);
        progress(limit, searched, begin.elapsed().as_millis());

        if let Some(node) = goal {
            return Some(to_solution(&node, stage));
        }
        if prev_searched == searched {
            return None;
        }

        prev_searched = searched;
    }

    None
}

fn dls(stage: &Stage, limit: u16) -> (usize, Option<Rc<Node>>) {
    let root = Node::root(stage);

    let mut nodes = vec![Rc::new(root)];
    let mut searched = 0usize;
    let mut children = Vec::new();

    while let Some(node) = nodes.pop() {
        searched += 1;

        if node.state.erased.contains(stage.goal()) {
            return (searched, Some(node));
        }

        if node.depth == limit {
            continue;
        }

        node.state.add_children(&mut children, stage);
        for child in children.drain(..) {
            let child_node = Node::child(&node, child);
            nodes.push(Rc::new(child_node));
        }
    }

    (searched, None)
}

fn progress(limit: u16, searched: usize, ms: u128) {
    eprintln!("limit: {limit} searched: {searched} timeMs: {ms}");
}

fn to_solution(goal: &Rc<Node>, stage: &Stage) -> Solution {
    let mut actions = Vec::new();
    let mut current = Some(Rc::clone(goal));

    while let Some(n) = current.take() {
        actions.push(n.action);
        current.clone_from(&n.parent);
    }

    actions.reverse();

    Solution::new(goal.state.status, &actions, stage)
}
