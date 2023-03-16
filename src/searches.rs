use crate::traits::*;

use std::collections::BinaryHeap;

pub fn a_star<T: Searchable>(start: T) -> T {
    let mut heap: BinaryHeap<T> = BinaryHeap::new();

    let mut next = start;

    while next.heuristic() != 0 {

        for succ in next.generate_successors() {
            heap.push(succ);
        }

        next = heap.pop().unwrap(); 
    }

    next
}

pub fn ida_star<T: Searchable>(start: T) -> T {

    let mut bound = start.heuristic();

    loop {
        let res = search(start.clone(), bound);
        if res.0 == 0 {
            return res.1;
        } else {
            bound = res.0;
        }
    }

    fn search<T: Searchable>(next: T, bound: u16) -> (u16, T) {
        if next.f() > bound {
            return (next.f(), next);
        }
        
        let mut min = (u16::MAX, next.clone());

        for succ in next.generate_successors() {
            let t = search(succ, bound);
            if t.1.heuristic() == 0 {
                return (0, t.1)
            } else if t.0 < min.0 {
                min = t;
            }
        }

        min
    }

}


pub fn rbf_search<T: Searchable>(start: T) -> T {

    fn search<T: Searchable>(next: T, f: u16, limit: u16) -> (u16, bool, T) {

        if next.heuristic() == 0 {
            return (0, true, next);
        }

        let mut succs = next.generate_successors()
            .into_iter().map(|s| (s.f(), s)).collect::<Vec<(u16, T)>>();

        if succs.len() == 0 {
            return (u16::MAX, false, next)
        }

        succs.sort_by_key(|s| u16::MAX - s.0);

        for mut succ in succs.iter_mut() {
            succ.0 = std::cmp::max(succ.0, f);
        }

        loop {
            let mut best = match succs.pop() {
                Some(b) => b,
                None => unreachable!()
            };

            if best.0 > limit {
                return (best.0, false, best.1);
            };

            let alt_min = match succs.last() {
                Some(b) => std::cmp::min(limit, b.0),
                None => limit
            };

            let res = search(best.1.clone(), best.0, alt_min);

            best.0 = res.0;
            succs.push(best);
            succs.sort_by_key(|s| u16::MAX - s.0);

            if res.1 {
                return res;
            }
        }

    }

    let f = start.f();
    let res = search(start, f, u16::MAX);

    res.2

}