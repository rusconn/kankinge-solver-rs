use crate::object::{Enemy, GateKind, U8UpKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Status {
    pub hp: u16,
    pub atk: u8,
    pub def: u8,
    pub gold: u8,
    pub silver: u8,
    pub blue: u8,
    pub mag: u8,
    pub level: u8,
    pub crystal: u8,
}

impl Status {
    pub(crate) fn initial() -> Self {
        Self {
            hp: 150,
            atk: 10,
            def: 0,
            gold: 0,
            silver: 0,
            blue: 0,
            mag: 0,
            level: 0,
            crystal: 0,
        }
    }

    pub(crate) fn get_u8_up(&self, kind: U8UpKind, amount: u8) -> Self {
        let mut after = *self;
        after.add_u8_amount(kind, amount);
        after
    }

    pub(crate) fn add_u8_amount(&mut self, kind: U8UpKind, amount: u8) {
        match kind {
            U8UpKind::Atk => self.atk += amount,
            U8UpKind::Def => self.def += amount,
            U8UpKind::Gold => self.gold += amount,
            U8UpKind::Silver => self.silver += amount,
            U8UpKind::Blue => self.blue += amount,
            U8UpKind::Crystal => self.crystal += amount,
        }
    }

    pub(crate) fn get_hp_up(&self, amount: u16) -> Self {
        let mut after = *self;
        after.add_hp_amount(amount);
        after
    }

    pub(crate) fn add_hp_amount(&mut self, amount: u16) {
        self.hp += amount;
    }

    pub(crate) fn try_open_gate(&self, kind: GateKind) -> Option<Self> {
        if self.key_count(kind) == 0 {
            return None;
        }
        let mut after = *self;
        after.open_gate(kind);
        Some(after)
    }

    fn key_count(&self, kind: GateKind) -> u8 {
        match kind {
            GateKind::Gold => self.gold,
            GateKind::Silver => self.silver,
            GateKind::Blue => self.blue,
        }
    }

    fn open_gate(&mut self, kind: GateKind) {
        match kind {
            GateKind::Gold => self.gold -= 1,
            GateKind::Silver => self.silver -= 1,
            GateKind::Blue => self.blue -= 1,
        }
    }

    pub(crate) fn is_no_dmg(&self, enemy: &Enemy) -> bool {
        self.damage_from(enemy) == Some(0)
    }

    pub(crate) fn try_battle(&self, enemy: &Enemy) -> Option<Self> {
        let dmg = self.damage_from(enemy)?;
        if dmg >= self.hp.into() {
            return None;
        }
        let mut after = *self;
        after.battle(dmg.try_into().expect("u16のhpよりも小さいことを検証済"));
        Some(after)
    }

    fn damage_from(&self, enemy: &Enemy) -> Option<u32> {
        let dpt = self.atk.saturating_sub(enemy.def);
        if dpt == 0 {
            return None;
        }
        let turn = enemy.hp.div_ceil(dpt.into());
        let dmg_count = turn - 1;
        let edpt = enemy.atk.saturating_sub(self.def.into());
        Some(u32::from(edpt) * u32::from(dmg_count))
    }

    fn battle(&mut self, dmg: u16) {
        self.hp -= dmg;
        self.mag += 1;
    }

    pub(crate) fn try_convert_silver(&self) -> Option<Self> {
        if self.mag < 60 {
            return None;
        }
        let mut after = *self;
        after.convert_silver();
        Some(after)
    }

    fn convert_silver(&mut self) {
        self.mag -= 60;
        self.silver += 1;
        self.level += 3;
    }

    pub(crate) fn try_convert_hp_atk_def(&self) -> Option<(Self, Self, Self)> {
        if self.mag < 40 {
            return None;
        }
        let mut hp = *self;
        hp.convert_hp();
        let mut atk = *self;
        atk.convert_atk();
        let mut def = *self;
        def.convert_def();
        Some((hp, atk, def))
    }

    fn convert_hp(&mut self) {
        self.mag -= 40;
        self.hp += 500 + 150 * u16::from(self.level);
        self.level += 2;
    }

    fn convert_atk(&mut self) {
        self.mag -= 40;
        self.atk += 5 + self.level;
        self.level += 2;
    }

    fn convert_def(&mut self) {
        self.mag -= 40;
        self.def += 5 + self.level;
        self.level += 2;
    }

    pub(crate) fn try_convert_gold(&self) -> Option<Self> {
        if self.mag < 20 {
            return None;
        }
        let mut after = *self;
        after.convert_gold();
        Some(after)
    }

    fn convert_gold(&mut self) {
        self.mag -= 20;
        self.gold += 1;
        self.level += 1;
    }

    pub(crate) fn compare(&self, other: &Self) -> StatusCmp {
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

        check!(self.hp, other.hp);
        check!(self.atk, other.atk);
        check!(self.def, other.def);
        check!(self.gold, other.gold);
        check!(self.silver, other.silver);
        check!(self.blue, other.blue);
        check!(self.mag, other.mag);
        check!(self.level, other.level);
        check!(self.crystal, other.crystal);

        if self_gt {
            StatusCmp::Greater
        } else if other_gt {
            StatusCmp::Less
        } else {
            StatusCmp::Equal
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StatusCmp {
    Equal,
    Greater,
    Less,
    Incomparable,
}
