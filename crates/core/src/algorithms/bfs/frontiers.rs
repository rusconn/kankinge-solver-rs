use rustc_hash::FxHashMap;

use crate::{algorithms::bfs::ArenaIndex, state::StatusCmp};

use super::Arena;

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

            match node.state.compare_status(&frontier_node.state) {
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
