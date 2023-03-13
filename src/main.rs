mod consts;
mod node;
mod linktree;
mod searches;

use std::collections::BinaryHeap;

fn main() {
    let mut heap: BinaryHeap<node::SearchNode> = BinaryHeap::new();
    heap.push(node::start_node());

    let mut next = heap.pop().unwrap();

    //println!("{:?}", lehmer_code([1,2,4,3]));
    //panic!();


    ///* 
    while next.heuristic != 0 {

        //println!("{:?} {:?} {:?}", linktree::get_sequence(next.treenode.clone()), next.heuristic, next.cost);

        for succ in next.generate_successors() {
            heap.push(succ);
        }

        //if linktree::get_sequence(next.treenode.clone()) == vec![1,2,3,4,1,2,3,1,4] {
        //    println!("{:?}", next.generate_successors().into_iter().map(|s| s.symbol).collect::<Vec<u8>>());
        //}

        next = heap.pop().unwrap();
    }//*/

    println!("{:?}", linktree::get_sequence(next.treenode));
}
