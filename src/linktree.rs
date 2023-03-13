use std::rc::Rc;

#[derive(Debug)]
pub struct LinkTree {
    pub parent: Option<Rc<LinkTree>>,
    pub symbol: u8
}

pub fn get_sequence(end: Rc<LinkTree>) -> Vec<u8> {
    let mut ret = Vec::new();

    let mut curr = Some(end);

    while let Some(node) = curr {
        ret.push(node.symbol);
        curr = node.parent.clone();
    }

    ret.reverse();
    ret
}
