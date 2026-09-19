use std::hash::Hasher;

use fixedbitset::FixedBitSet;
use rustc_hash::FxHasher;

use crate::object::{Enemy, Object};

use super::{GridIndex, Stage};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Instance {
    pub(crate) id: InstanceId,
    pub(crate) object: &'static Object,
    pub(super) index: GridIndex,
}

impl Instance {
    pub(crate) fn new(id: InstanceId, index: usize, object: &'static Object) -> Self {
        Self {
            id,
            index: index.into(),
            object,
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        self.object.name()
    }

    pub(crate) fn is_up(&self) -> bool {
        self.object.is_u_8_up() || self.object.is_hp_up()
    }

    pub(crate) fn is_goal(&self) -> bool {
        self.object.is_goal()
    }

    pub(crate) fn as_enemy(&self) -> Option<&Enemy> {
        self.object.as_enemy()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct InstanceId(u16);

impl TryFrom<usize> for InstanceId {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u16::try_from(value).map(Self).map_err(|_| "大きすぎ")
    }
}

impl InstanceId {
    pub(crate) const ZERO: Self = Self(0);

    pub(crate) fn as_usize(self) -> usize {
        self.0.into()
    }

    // オーバーフローは非現実的なので考慮していない
    pub(crate) fn succ(self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct InstanceCount(usize);

impl TryFrom<usize> for InstanceCount {
    type Error = &'static str;

    fn try_from(count: usize) -> Result<Self, Self::Error> {
        // idは0始まりなので，最後のidはcount-1
        InstanceId::try_from(count.saturating_sub(1))
            .map(|_| Self(count))
            .map_err(|_| "インスタンス数が大きすぎ")
    }
}

impl InstanceCount {
    fn as_usize(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InstanceSet(FixedBitSet);

impl InstanceSet {
    pub(super) fn new(count: InstanceCount) -> Self {
        Self(FixedBitSet::with_capacity(count.as_usize()))
    }

    // Hashは実装しない。Frontiersのための最適化。
    // FixedBitSetのハッシュコストはΘ(words)でありそこそこ重い。
    // HashMapキーへ利用すると拡張時の全キー再ハッシュが高くつくので、ハッシュコストの小さいu64へ変換している。
    pub(crate) fn hash_value(&self) -> u64 {
        let mut hasher = FxHasher::default();
        for word in self.0.as_slice() {
            hasher.write_usize(*word);
        }
        hasher.finish()
    }

    pub(crate) fn contains(&self, instance: &Instance) -> bool {
        self.0.contains(instance.id.as_usize())
    }

    pub(crate) fn iter<'a>(&'a self, stage: &'a Stage) -> impl Iterator<Item = &'a Instance> {
        self.0
            .ones()
            .map(|bit| stage.instance_of(bit.try_into().expect("InstanceCountで検証済")))
    }

    pub(crate) fn insert(&mut self, instance: &Instance) {
        self.0.insert(instance.id.as_usize());
    }

    pub(crate) fn remove(&mut self, instance: &Instance) {
        self.0.remove(instance.id.as_usize());
    }

    pub(crate) fn difference_with(&mut self, other: &Self) {
        self.0.difference_with(&other.0);
    }

    pub(crate) fn union_with(&mut self, other: &Self) {
        self.0.union_with(&other.0);
    }
}
