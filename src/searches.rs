use crate::*;

use std::collections::BinaryHeap;

pub fn a_star(start: node::SearchNode) -> Option<node::SearchNode> {
    let mut heap: BinaryHeap<node::SearchNode> = BinaryHeap::new();

    let mut next = start;

    while next.heuristic != 0 {

        for succ in next.generate_successors() {
            heap.push(succ);
        }

        next = match heap.pop() {
            Some(g) => g,
            None => return None
        };
    }

    return Some(next);
}