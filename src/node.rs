use std::cmp::Ordering;
use std::rc::Rc;
use std::iter::*;
use crate::consts::*;
use crate::traits::*;
use crate::*;

#[derive(Clone, Debug)]
pub struct SearchNode {
    pub treenode: Rc<linktree::LinkTree>,
    pub symbol: u8,
    pub heuristic: u16,
    pub f: u16,
    pub cycles: [u8; CYCLES],
    pub suffix: [u8; (N as usize)],
    pub wasted: u8
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        (self.symbol == other.symbol) && Rc::ptr_eq(&self.treenode, &other.treenode)
    }
}

impl Eq for SearchNode {}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.f).cmp(&self.f)
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn lehmer_code(perm: [u8; (N as usize)]) -> usize {
    let mut code = 0;

    for i in 1..N {
        let mut temp: usize = 0;
        for j in (i+1)..N {
            if perm[j as usize] < perm[i as usize] {
                temp += 1;
            }
        }

        code += temp * factorial((N-i-1) as usize);
    }

    code
}

pub fn start_node() -> SearchNode {
    let mut treenode = linktree::LinkTree { parent: None, symbol: 1 };

    let mut suffix: [u8; N as usize] = [1; N as usize];
    for i in 2..=N {
        suffix[(i as usize) - 1] = i as u8;
        treenode = linktree::LinkTree { parent: Some(Rc::new(treenode)), symbol: i as u8 };
    }

    let mut cycles = [0; CYCLES];
    cycles[lehmer_code(suffix)] = MASK;
    suffix.rotate_right(1);

    for i in 1..N {
        treenode = linktree::LinkTree { parent: Some(Rc::new(treenode)), symbol: i as u8 };
    }

    let h = (factorial(N.into()) + factorial((N-1).into()) + factorial((N-2).into())) as u16 - ((N+1) as u16);

    SearchNode {
        treenode: Rc::new(treenode),
        cycles: cycles,
        symbol: (N-1) as u8,
        heuristic: h,
        f: (6*N - 3) as u16 + h,
        suffix: suffix,
        wasted: 0
    }
}

pub fn is_permutation(perm: [u8; (N as usize)]) -> bool {
    let mut mask: u8 = 0;

    for i in 0..=N-1 {
        mask |= 1 << perm[i as usize];
    }

    (mask >> 1) == MASK
}

impl Searchable for SearchNode {
    fn generate_successors(&self) -> Vec<SearchNode> {
        let mut ret = Vec::with_capacity(N as usize);

        for i in 1..=N {
            if i as u8 == self.symbol {
                continue;
            }

            let mut delta = 0;

            let treenode: linktree::LinkTree = linktree::LinkTree {
                symbol: i as u8,
                parent: Some(self.treenode.clone())
            };

            let mut suffix = self.suffix.clone();
            suffix.rotate_left(1);
            suffix[(N-1) as usize] = i as u8;

            let mut cycles = self.cycles.clone();
            let mut wasted = self.wasted;

            if is_permutation(suffix) {
                wasted = 0;

                //println!("{:?}", suffix);

                let mut cycle_suffix = suffix.clone();
                while cycle_suffix[0] != 1 {
                    cycle_suffix.rotate_left(1);
                }

                let code = lehmer_code(cycle_suffix);

                if cycles[code] & (1 << (suffix[0]) as usize - 1) == 0 {
                    /* We've discovered a new permutation */
                    delta += 1;
                    cycles[code] |= 1 << (suffix[0]) as usize - 1;

                    if cycles[code] & MASK == MASK {
                        /* We've completed an entire cycle */
                        delta += 1;

                        let mut count = 0;
                        let mut succs = successors(Some(suffix), move |_| {
                            count += 1;

                            if count < N {
                                let mut new_suffix = suffix.clone();
                                new_suffix.rotate_right(1);
                                let temp = new_suffix[(N-2) as usize];
                                new_suffix[(N-2) as usize] = new_suffix[(N-1) as usize];
                                new_suffix[(N-1) as usize] = temp;

                                while new_suffix[0] != 1 {
                                    new_suffix.rotate_right(1);
                                }

                                return Some(new_suffix);
                            } else {
                                return None
                            };
                        });

                        if succs.all(|perm| cycles[lehmer_code(perm)] & MASK == MASK) {
                            /* We have completed a 2-loop */
                            delta += 1
                        }
                    }
                } else {
                    /* This is not a new permutation, continue */
                    continue;
                }
            } else {
                wasted += 1;

                if wasted == N as u8 {
                    continue;
                } else if (consts::MAX * 3) < (self.f + 3).into() {
                    continue;
                } else {
                    let mut prevs: [usize; N] = [N+1; N];

                    let mut prev = 0;
                    let mut i = 0;
                    let mut skip = false;
                    let mut next_prev = 0;

                    while i < N {
                        while i < N {
                            //println!("??? {:?} {:?}", suffix[i], i);
                            if prevs[suffix[i] as usize - 1] != N+1 {
                                next_prev = prevs[suffix[i] as usize - 1] + 1;
                                prev = i - 1;
                                i += 1;
                                
                                break;
                            } else {
                                prevs[suffix[i] as usize - 1] = i;
                            }
                            i += 1;
                        }

                        while i < N {
                            if suffix[i] != suffix[next_prev] {
                                i += 1;
                                break;
                            } else {
                                //println!("what can i do? {:?} {:?}", next_prev, prev);
                                if next_prev == prev {
                                    skip = true;
                                    break;
                                }
                                i += 1;
                                next_prev += 1;
                                
                            }
                        }

                        if skip {
                            break;
                        }
                    }

                    if skip {
                        //println!("{:?}", suffix);
                        continue;
                    }
                }
            }

            ret.push(SearchNode {
                treenode: Rc::new(treenode),
                f: self.f + 3 - delta,
                heuristic: self.heuristic - delta,
                symbol: i as u8,
                cycles: cycles,
                suffix: suffix,
                wasted: wasted
            });

        };

        ret
    }

    #[inline(always)]
    fn f(&self) -> u16 {
        self.f
    }

    #[inline(always)]
    fn heuristic(&self) -> u16 {
        self.heuristic
    }
}
