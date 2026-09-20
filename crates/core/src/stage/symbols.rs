use phf::phf_map;

use crate::object::{Enemy, Object, OneUpKind};

use super::Cell;

pub(super) static MAP: phf::Map<char, Cell> = phf_map! {
    'a' => Cell::Object(&Object::Enemy(Enemy {
        name: "爺",
        hp: 6358,
        atk: 12,
        def: 10,
    })),
    'b' => Cell::Object(&Object::Enemy(Enemy {
        name: "緑何か",
        hp: 230,
        atk: 17,
        def: 0,
    })),
    'c' => Cell::Object(&Object::Enemy(Enemy {
        name: "赤トゲトゲ",
        hp: 250,
        atk: 14,
        def: 1,
    })),
    'd' => Cell::Object(&Object::Enemy(Enemy {
        name: "青ツンツン",
        hp: 300,
        atk: 14,
        def: 0,
    })),
    'e' => Cell::Object(&Object::Enemy(Enemy {
        name: "サソリ",
        hp: 230,
        atk: 13,
        def: 2,
    })),
    'f' => Cell::Object(&Object::Enemy(Enemy {
        name: "青羽",
        hp: 270,
        atk: 13,
        def: 1,
    })),
    'g' => Cell::Object(&Object::Enemy(Enemy {
        name: "草マン",
        hp: 250,
        atk: 10,
        def: 3,
    })),
    'h' => Cell::Object(&Object::HpUp {
        name: "hp800",
    }),
    'i' => Cell::Object(&Object::OneUp {
        kind: OneUpKind::Atk,
        name: "atk1",
    }),
    'j' => Cell::Object(&Object::OneUp {
        kind: OneUpKind::Def,
        name: "def1",
    }),

    '□' | '■' => Cell::Wall,

    ' ' => Cell::Road,

    '@' => Cell::Start,

    '◯' => Cell::Object(&Object::Goal {
        name: "goal"
    }),
};
