use bitfield_struct::bitfield;

use crate::object::{Enemy, OneUpKind};

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct Status {
    #[bits(14)]
    pub hp: u16,

    #[bits(4)]
    pub atk: u8,

    #[bits(4)]
    pub def: u8,

    #[bits(10)]
    __: u16,
}

const _: () = assert!(size_of::<Status>() == 4);

impl Status {
    pub(crate) fn initial() -> Self {
        Self::new().with_hp(1000).with_atk(5).with_def(5)
    }

    pub(crate) fn get_one_up(self, kind: OneUpKind) -> Self {
        match kind {
            OneUpKind::Atk => self.with_atk(self.atk() + 1),
            OneUpKind::Def => self.with_def(self.def() + 1),
        }
    }

    pub(crate) fn get_hp_up(self) -> Self {
        self.with_hp(self.hp() + 800)
    }

    pub(crate) fn is_no_dmg(self, enemy: &Enemy) -> bool {
        self.damage_from(enemy) == Some(0)
    }

    pub(crate) fn try_battle(self, enemy: &Enemy) -> Option<Self> {
        let dmg = self.damage_from(enemy)?;
        if dmg >= self.hp().into() {
            return None;
        }

        let dmg: u16 = dmg.try_into().expect("u16のhpよりも小さいことを検証済");

        Some(self.with_hp(self.hp() - dmg))
    }

    fn damage_from(self, enemy: &Enemy) -> Option<u32> {
        let dpt = self.atk().saturating_sub(enemy.def);
        if dpt == 0 {
            return None;
        }

        let turn = enemy.hp.div_ceil(dpt.into());
        let dmg_count = turn - 1;
        let edpt = enemy.atk.saturating_sub(self.def());

        Some(u32::from(edpt) * u32::from(dmg_count))
    }

    pub(crate) fn compare(self, other: Self) -> StatusCmp {
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

        check!(self.hp(), other.hp());
        check!(self.atk(), other.atk());
        check!(self.def(), other.def());

        if self_gt {
            StatusCmp::Greater
        } else if other_gt {
            StatusCmp::Less
        } else {
            StatusCmp::Equal
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Status", 9)?;
        state.serialize_field("hp", &self.hp())?;
        state.serialize_field("atk", &self.atk())?;
        state.serialize_field("def", &self.def())?;
        state.end()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StatusCmp {
    Equal,
    Greater,
    Less,
    Incomparable,
}
