mod arena;
mod frontiers;
mod node;

use std::{mem, time::Instant};

use crate::{Solution, stage::Stage};

use arena::{Arena, ArenaIndex};
use frontiers::Frontiers;
use node::Node;

pub(crate) fn run(stage: &Stage, skip_drop: bool) -> Option<Solution> {
    let root = Node::root(stage);

    let mut arena = Arena::from([root]);
    let mut frontiers = Frontiers::new();
    let mut current = vec![ArenaIndex::FIRST];
    let mut next = Vec::new();
    let mut children = Vec::new();

    let mut solution = None;
    'search: for depth in 0u32.. {
        let mut searched = 0usize;
        let begin = Instant::now();

        for &idx in &current {
            if !arena[idx].alive.get() {
                continue;
            }

            searched += 1;

            if arena[idx].state.erased.contains(stage.goal()) {
                progress(depth, searched, begin.elapsed().as_millis());
                solution = Some(to_solution(&arena, idx, stage));
                break 'search;
            }

            arena[idx].state.add_children(&mut children, stage);
            for child in children.drain(..) {
                let child_idx = arena.next_index();
                arena.push(Node::child(idx, arena[idx].depth, child));
                if frontiers.offer(&arena, child_idx) {
                    next.push(child_idx);
                } else {
                    arena.pop();
                }
            }
        }

        progress(depth, searched, begin.elapsed().as_millis());

        if next.is_empty() {
            break;
        }

        mem::swap(&mut current, &mut next);
        next.clear();
    }

    if skip_drop {
        mem::forget((arena, frontiers, current, next, children));
    }

    solution
}

fn progress(depth: u32, searched: usize, ms: u128) {
    eprintln!("depth: {depth} searched: {searched} timeMs: {ms}");
}

fn to_solution(arena: &Arena, goal_idx: ArenaIndex, stage: &Stage) -> Solution {
    let mut moved_tos = Vec::new();
    let mut idx = Some(goal_idx);

    while let Some(i) = idx
        && let node = &arena[i]
        && let Some(moved_to) = node.moved_to
    {
        moved_tos.push(moved_to);
        idx = node.parent;
    }

    moved_tos.reverse();

    Solution::new(arena[goal_idx].state.status, &moved_tos, stage)
}
