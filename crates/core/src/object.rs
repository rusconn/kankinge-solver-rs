use strum::EnumIs;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIs)]
pub(crate) enum Object {
    Enemy(Enemy),
    U8Up {
        kind: U8UpKind,
        name: &'static str,
        amount: u8,
    },
    HpUp {
        name: &'static str,
        amount: u16,
    },
    Gate {
        kind: GateKind,
        name: &'static str,
    },
    Goal {
        name: &'static str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Enemy {
    pub(crate) name: &'static str,
    pub(crate) hp: u16,
    pub(crate) atk: u16,
    pub(crate) def: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum U8UpKind {
    Atk,
    Def,
    Gold,
    Silver,
    Blue,
    Crystal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GateKind {
    Gold,
    Silver,
    Blue,
}

impl Object {
    pub(crate) fn as_enemy(&self) -> Option<&Enemy> {
        match self {
            Self::Enemy(enemy) => Some(enemy),
            _ => None,
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        match self {
            Self::Enemy(enemy) => enemy.name,
            Self::U8Up { name, .. }
            | Self::HpUp { name, .. }
            | Self::Gate { name, .. }
            | Self::Goal { name } => name,
        }
    }
}
