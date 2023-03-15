mod consts;
mod node;
mod linktree;
mod searches;

use std::time::{Duration, Instant};

fn main() {
    let start = node::start_node();

    let astar_start = Instant::now();
    let astar = searches::a_star(start.clone());
    let astar_time = astar_start.elapsed();

    let ida_start = Instant::now();
    let ida = searches::ida_star(start.clone());
    let ida_time = ida_start.elapsed();

    let rbfs_start = Instant::now();
    let rbfs = searches::rbf_search(start);
    let rbfs_time = rbfs_start.elapsed();

    println!("A* ({:?}): {:?}", consts::N, astar_time);
    println!("IDA* ({:?}): {:?}", consts::N, ida_time);
    println!("RBFS ({:?}): {:?}", consts::N, rbfs_time);
    //println!("A-star: {:?}")
}
