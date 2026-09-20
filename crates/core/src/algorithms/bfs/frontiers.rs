use rustc_hash::FxHashMap;

use crate::Status;

use super::{Arena, ArenaIndex};

#[derive(Debug, Default)]
pub(crate) struct Frontiers {
    map: FxHashMap<u64, Vec<ArenaIndex>>,
}

// 下記構造体をmapのキーに使おうかと思ったが、遅かったので避けた。
// Eq実装へofferの衝突ガードを移せる等メリットがあったのだが。
//
// struct FrontiersKey {
//     erased: Rc<InstanceSet>,
//     hash: u64,
// }

impl Frontiers {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn offer(&mut self, arena: &Arena, idx: ArenaIndex) -> bool {
        let node = &arena[idx];
        let key = node.state.erased.hash_value();
        let frontiers = self.map.entry(key).or_default();

        let mut i = 0;
        while i < frontiers.len() {
            let frontier_node = &arena[frontiers[i]];

            // ハッシュ衝突時用の内容一致ガード
            if node.state.erased != frontier_node.state.erased {
                i += 1;
                continue;
            }

            match compare(node.state.status, frontier_node.state.status) {
                StatusCmp::Equal | StatusCmp::Less => {
                    return false;
                }
                StatusCmp::Greater => {
                    frontier_node.alive.set(false);
                    frontiers.swap_remove(i);
                }
                StatusCmp::Incomparable => {
                    i += 1;
                }
            }
        }

        frontiers.push(idx);
        true
    }
}

fn compare(a: Status, b: Status) -> StatusCmp {
    let mut self_gt = false;
    let mut other_gt = false;

    macro_rules! check {
        ($a:expr, $b:expr) => {
            if $a != $b {
                if $a > $b {
                    self_gt = true;
                } else {
                    other_gt = true;
                }
                if self_gt && other_gt {
                    return StatusCmp::Incomparable;
                }
            }
        };
    }

    check!(a.hp(), b.hp());
    check!(a.atk(), b.atk());
    check!(a.def(), b.def());

    if self_gt {
        StatusCmp::Greater
    } else if other_gt {
        StatusCmp::Less
    } else {
        StatusCmp::Equal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusCmp {
    Equal,
    Greater,
    Less,
    Incomparable,
}
