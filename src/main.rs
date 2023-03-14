mod consts;
mod node;
mod linktree;
mod searches;


fn main() {
    let res = searches::ida_star(node::start_node());
    println!("{:?}", res.map(|s| linktree::get_sequence(s.treenode) ));
}
