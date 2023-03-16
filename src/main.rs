mod consts;
mod node;
mod linktree;
mod searches;
mod traits;

use std::time::Instant;

fn main() {
    let start = node::start_node();

    let astar_start = Instant::now();
    let _astar = searches::a_star(start.clone());
    let astar_time = astar_start.elapsed();

    let ida_start = Instant::now();
    let _ida = searches::ida_star(start.clone());
    let ida_time = ida_start.elapsed();

    let rbfs_start = Instant::now();
    let _rbfs = searches::rbf_search(start);
    let rbfs_time = rbfs_start.elapsed();

    println!("A* (n={:?}): {:?}", consts::N, astar_time);
    println!("IDA* (n={:?}): {:?}", consts::N, ida_time);
    println!("RBFS (n={:?}): {:?}", consts::N, rbfs_time);
    //println!("A-star: {:?}")
}
