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

pub fn ida_star(start: node::SearchNode) -> Option<node::SearchNode> {

    let mut bound = start.heuristic;

    loop {
        let res = search(start.clone(), bound);
        if res.0 == u16::MAX {
            return None;
        } else if res.0 == 0 {
            return Some(res.1);
        } else {
            bound = res.0;
        }

        println!("branch is now: {:?}", bound);
    }

    fn search(next: node::SearchNode, bound: u16) -> (u16, node::SearchNode) {
        if next.f > bound {
            return (next.f, next);
        }
        
        let mut min = (u16::MAX, next.clone());

        for succ in next.generate_successors() {
            let t = search(succ, bound);
            if t.1.heuristic == 0 {
                return (0, t.1)
            } else if t.0 < min.0 {
                min = t;
            }
        }

        min
    }

}

/* 
pub fn rbf_search(start: node::SearchNode) -> Option<node::SearchNode> {

    fn search(next: node::SearchNode, limit: u16) -> ((u16, u16), node::SearchNode) {

        if next.heuristic == 0 {
            return ((next.cost, 0), next);
        }
        
        let mut min = ((next.cost, next.heuristic), next.clone());

        let mut succs = next.generate_successors();
        succs.sort();

        loop {
            let mut best = match succs.pop() {
                Some(b) => b,
                None => return ((u16::MAX, u16::MAX), next)
            }

            
        }

        min
    }

}*/