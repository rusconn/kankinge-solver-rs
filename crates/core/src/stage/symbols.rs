use phf::phf_map;

use crate::object::{Enemy, GateKind, Object, U8UpKind};

use super::Cell;

pub(super) static MAP: phf::Map<char, Cell> = phf_map! {
    'a' => Cell::Object(&Object::Enemy(Enemy {
        name: "ゴブリン",
        hp: 30,
        atk: 20,
        def: 0,
    })),
    'b' => Cell::Object(&Object::Enemy(Enemy {
        name: "フードゴブリン",
        hp: 35,
        atk: 10,
        def: 5,
    })),
    'c' => Cell::Object(&Object::Enemy(Enemy {
        name: "メットゴブリン",
        hp: 75,
        atk: 10,
        def: 5,
    })),
    'd' => Cell::Object(&Object::Enemy(Enemy {
        name: "青メットゴブリン",
        hp: 60,
        atk: 150,
        def: 5,
    })),
    'e' => Cell::Object(&Object::Enemy(Enemy {
        name: "ネコ",
        hp: 35,
        atk: 15,
        def: 10,
    })),
    'f' => Cell::Object(&Object::Enemy(Enemy {
        name: "ヘビ",
        hp: 20,
        atk: 30,
        def: 15,
    })),
    'g' => Cell::Object(&Object::Enemy(Enemy {
        name: "コウモリ",
        hp: 30,
        atk: 50,
        def: 0,
    })),
    'h' => Cell::Object(&Object::Enemy(Enemy {
        name: "白トリ",
        hp: 100,
        atk: 25,
        def: 0,
    })),
    'i' => Cell::Object(&Object::Enemy(Enemy {
        name: "赤トリ",
        hp: 60,
        atk: 70,
        def: 10,
    })),
    'j' => Cell::Object(&Object::Enemy(Enemy {
        name: "ヤギ",
        hp: 200,
        atk: 40,
        def: 0,
    })),
    'k' => Cell::Object(&Object::Enemy(Enemy {
        name: "青オオカミ",
        hp: 100,
        atk: 150,
        def: 0,
    })),
    'l' => Cell::Object(&Object::Enemy(Enemy {
        name: "白オオカミ",
        hp: 200,
        atk: 100,
        def: 0,
    })),
    'm' => Cell::Object(&Object::Enemy(Enemy {
        name: "赤ミノタウロス",
        hp: 600,
        atk: 30,
        def: 0,
    })),
    'n' => Cell::Object(&Object::Enemy(Enemy {
        name: "マスタードミノタウロス",
        hp: 200,
        atk: 50,
        def: 15,
    })),
    'o' => Cell::Object(&Object::Enemy(Enemy {
        name: "マントミノタウロス",
        hp: 600,
        atk: 75,
        def: 0,
    })),
    'p' => Cell::Object(&Object::Enemy(Enemy {
        name: "触手",
        hp: 50,
        atk: 40,
        def: 10,
    })),
    'q' => Cell::Object(&Object::Enemy(Enemy {
        name: "オオワシ",
        hp: 1000,
        atk: 40,
        def: 10,
    })),
    'r' => Cell::Object(&Object::Enemy(Enemy {
        name: "紫オオワシ",
        hp: 300,
        atk: 200,
        def: 50,
    })),
    's' => Cell::Object(&Object::Enemy(Enemy {
        name: "紫ハゲオーガ",
        hp: 900,
        atk: 100,
        def: 0,
    })),
    't' => Cell::Object(&Object::Enemy(Enemy {
        name: "赤オーガ",
        hp: 150,
        atk: 100,
        def: 35,
    })),
    'u' => Cell::Object(&Object::Enemy(Enemy {
        name: "緑オーガ",
        hp: 400,
        atk: 250,
        def: 80,
    })),
    'v' => Cell::Object(&Object::Enemy(Enemy {
        name: "赤スライム",
        hp: 50,
        atk: 15,
        def: 15,
    })),
    'w' => Cell::Object(&Object::Enemy(Enemy {
        name: "緑スライム",
        hp: 50,
        atk: 80,
        def: 40,
    })),
    'x' => Cell::Object(&Object::Enemy(Enemy {
        name: "黒スライム",
        hp: 100,
        atk: 180,
        def: 100,
    })),
    'y' => Cell::Object(&Object::Enemy(Enemy {
        name: "コボルト",
        hp: 150,
        atk: 350,
        def: 0,
    })),
    'z' => Cell::Object(&Object::Enemy(Enemy {
        name: "幽霊",
        hp: 200,
        atk: 60,
        def: 35,
    })),
    'A' => Cell::Object(&Object::Enemy(Enemy {
        name: "ヒツジ",
        hp: 200,
        atk: 75,
        def: 75,
    })),
    'B' => Cell::Object(&Object::Enemy(Enemy {
        name: "黄精霊",
        hp: 30,
        atk: 750,
        def: 80,
    })),
    'C' => Cell::Object(&Object::Enemy(Enemy {
        name: "紫精霊",
        hp: 100,
        atk: 500,
        def: 100,
    })),
    'D' => Cell::Object(&Object::Enemy(Enemy {
        name: "赤ゴーレム",
        hp: 1000,
        atk: 200,
        def: 50,
    })),
    'E' => Cell::Object(&Object::Enemy(Enemy {
        name: "青ゴーレム",
        hp: 2000,
        atk: 400,
        def: 0,
    })),
    'F' => Cell::Object(&Object::Enemy(Enemy {
        name: "人魂",
        hp: 100,
        atk: 250,
        def: 20,
    })),
    'G' => Cell::Object(&Object::Enemy(Enemy {
        name: "デビル",
        hp: 200,
        atk: 120,
        def: 30,
    })),

    'H' => Cell::Object(&Object::HpUp {
        name: "hp300",
        amount: 300,
    }),
    'I' => Cell::Object(&Object::HpUp {
        name: "hp600",
        amount: 600,
    }),
    'J' => Cell::Object(&Object::HpUp {
        name: "hp1200",
        amount: 1200,
    }),

    'K' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Atk,
        name: "atk1",
        amount: 1,
    }),
    'L' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Atk,
        name: "atk2",
        amount: 2,
    }),
    'M' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Atk,
        name: "atk4",
        amount: 4,
    }),
    'N' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Atk,
        name: "atk8",
        amount: 8,
    }),
    'O' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Atk,
        name: "atk16",
        amount: 16,
    }),

    'P' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Def,
        name: "def1",
        amount: 1,
    }),
    'Q' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Def,
        name: "def2",
        amount: 2,
    }),
    'R' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Def,
        name: "def4",
        amount: 4,
    }),
    'S' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Def,
        name: "def8",
        amount: 8,
    }),
    'T' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Def,
        name: "def16",
        amount: 16,
    }),

    'U' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Gold,
        name: "goldKey",
        amount: 1,
    }),
    'V' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Silver,
        name: "silverKey",
        amount: 1,
    }),
    'W' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Blue,
        name: "blueKey",
        amount: 1,
    }),

    'X' => Cell::Object(&Object::Gate {
        kind: GateKind::Gold,
        name: "goldGate",
    }),
    'Y' => Cell::Object(&Object::Gate {
        kind: GateKind::Silver,
        name: "silverGate",
    }),
    'Z' => Cell::Object(&Object::Gate {
        kind: GateKind::Blue,
        name: "blueGate",
    }),

    '◆' => Cell::Object(&Object::U8Up {
        kind: U8UpKind::Crystal,
        name: "crystal",
        amount: 1,
    }),

    '□' | '■' => Cell::Wall,

    ' ' => Cell::Road,

    '@' => Cell::Start,

    '◯' => Cell::Object(&Object::Goal {
        name: "goal"
    }),
};
