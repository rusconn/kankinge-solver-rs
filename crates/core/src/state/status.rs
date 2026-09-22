use bitfield_struct::bitfield;

use crate::object::{Enemy, GateKind, U8UpKind};

#[bitfield(u64)]
#[derive(PartialEq, Eq)]
pub struct Status {
    #[bits(15)]
    pub hp: u16,

    #[bits(8)]
    pub atk: u8,

    #[bits(8)]
    pub def: u8,

    #[bits(6)]
    pub gold: u8,

    #[bits(4)]
    pub silver: u8,

    #[bits(3)]
    pub blue: u8,

    #[bits(8)]
    pub mag: u8,

    #[bits(4)]
    pub level: u8,

    #[bits(2)]
    pub crystal: u8,

    #[bits(6)]
    __: u8,
}

const _: () = assert!(size_of::<Status>() == 8);

impl Status {
    pub(crate) fn initial() -> Self {
        Self::new().with_hp(150).with_atk(10)
    }

    pub(crate) fn get_u8_up(self, kind: U8UpKind, amount: u8) -> Self {
        match kind {
            U8UpKind::Atk => self.with_atk(self.atk() + amount),
            U8UpKind::Def => self.with_def(self.def() + amount),
            U8UpKind::Gold => self.with_gold(self.gold() + amount),
            U8UpKind::Silver => self.with_silver(self.silver() + amount),
            U8UpKind::Blue => self.with_blue(self.blue() + amount),
            U8UpKind::Crystal => self.with_crystal(self.crystal() + amount),
        }
    }

    pub(crate) fn get_hp_up(self, amount: u16) -> Self {
        self.with_hp(self.hp() + amount)
    }

    pub(crate) fn try_open_gate(self, kind: GateKind) -> Option<Self> {
        Some(match kind {
            GateKind::Gold => self.with_gold(self.gold().checked_sub(1)?),
            GateKind::Silver => self.with_silver(self.silver().checked_sub(1)?),
            GateKind::Blue => self.with_blue(self.blue().checked_sub(1)?),
        })
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

        Some(
            self.with_hp(self.hp() - dmg) //
                .with_mag(self.mag() + 1),
        )
    }

    fn damage_from(self, enemy: &Enemy) -> Option<u32> {
        let dpt = self.atk().saturating_sub(enemy.def);
        if dpt == 0 {
            return None;
        }

        let turn = enemy.hp.div_ceil(dpt.into());
        let dmg_count = turn - 1;
        let edpt = enemy.atk.saturating_sub(self.def().into());

        Some(u32::from(edpt) * u32::from(dmg_count))
    }

    pub(crate) fn try_convert_silver(self) -> Option<Self> {
        Some(
            self.with_mag(self.mag().checked_sub(60)?)
                .with_silver(self.silver() + 1)
                .with_level(self.level() + 3),
        )
    }

    pub(crate) fn try_convert_hp_atk_def(self) -> Option<(Self, Self, Self)> {
        let base = self
            .with_mag(self.mag().checked_sub(40)?)
            .with_level(self.level() + 2);

        Some((
            base.with_hp(self.hp() + 500 + 150 * u16::from(self.level())),
            base.with_atk(self.atk() + 5 + self.level()),
            base.with_def(self.def() + 5 + self.level()),
        ))
    }

    pub(crate) fn try_convert_gold(self) -> Option<Self> {
        Some(
            self.with_mag(self.mag().checked_sub(20)?)
                .with_gold(self.gold() + 1)
                .with_level(self.level() + 1),
        )
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
        state.serialize_field("gold", &self.gold())?;
        state.serialize_field("silver", &self.silver())?;
        state.serialize_field("blue", &self.blue())?;
        state.serialize_field("mag", &self.mag())?;
        state.serialize_field("level", &self.level())?;
        state.serialize_field("crystal", &self.crystal())?;
        state.end()
    }
}
