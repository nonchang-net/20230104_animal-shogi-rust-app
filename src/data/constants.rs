use super::{
    types::{Cell, Position},
    enums::{
        Koma,
        Side
    }
};

/**
 * constants.rs
 * - 定数置き場
 */


// 初期の盤状態
pub const INITIAL_BOARD_DATA: [[Cell; 3]; 4] = [
    [
        Cell{side: Side::B, koma: Koma::Kirin},
        Cell{side: Side::B, koma: Koma::Lion},
        Cell{side: Side::B, koma: Koma::Zou}
    ],
    [
        Cell{side: Side::Free, koma: Koma::Null},
        Cell{side: Side::B, koma: Koma::Hiyoko},
        Cell{side: Side::Free, koma: Koma::Null}
    ],
    [
        Cell{side: Side::Free, koma: Koma::Null},
        Cell{side: Side::A, koma: Koma::Hiyoko},
        Cell{side: Side::Free, koma: Koma::Null}
    ],
    [
        Cell{side: Side::A, koma: Koma::Zou},
        Cell{side: Side::A, koma: Koma::Lion},
        Cell{side: Side::A, koma: Koma::Kirin}
    ]
];


// 駒の移動できる方向定義一覧
// - Side::A（下）から見た時のpositionの配列
pub const KOMA_MOVERULE_HIYOKO: [Position; 1] = [
    Position{x:0, y:-1}
];
pub const KOMA_MOVERULE_KIRIN: [Position; 4] = [
    Position{x: 0, y:-1},
    Position{x: 0, y: 1},
    Position{x:-1, y: 0},
    Position{x: 1, y: 0},
];
pub const KOMA_MOVERULE_ZOU: [Position; 4] = [
    Position{x:-1, y:-1},
    Position{x:-1, y: 1},
    Position{x: 1, y:-1},
    Position{x: 1, y: 1},
];
pub const KOMA_MOVERULE_LION: [Position; 8] = [
    Position{x: 0, y:-1},
    Position{x: 0, y: 1},
    Position{x:-1, y: 0},
    Position{x: 1, y: 0},
    Position{x:-1, y:-1},
    Position{x:-1, y: 1},
    Position{x: 1, y:-1},
    Position{x: 1, y: 1},
];
pub const KOMA_MOVERULE_NIWATORI: [Position; 6] = [
    Position{x: 0, y:-1}, // 上
    Position{x:-1, y: 0}, // 左
    Position{x: 1, y: 0}, // 右
    Position{x:-1, y:-1}, // 左上
    Position{x: 1, y:-1}, // 右上
    Position{x: 0, y: 1}, // 下
];
