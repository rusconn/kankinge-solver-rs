use strum::EnumIs;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIs)]
pub(crate) enum Object {
    Enemy(Enemy),
    OneUp { kind: OneUpKind, name: &'static str },
    HpUp { name: &'static str },
    Goal { name: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Enemy {
    pub(crate) name: &'static str,
    pub(crate) hp: u16,
    pub(crate) atk: u8,
    pub(crate) def: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum OneUpKind {
    Atk,
    Def,
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
            Self::OneUp { name, .. } | Self::HpUp { name, .. } | Self::Goal { name } => name,
        }
    }
}
