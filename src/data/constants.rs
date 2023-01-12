use super::{
    types::Cell,
    enums::{
        Koma,
        Side
    }
};

/**
 * constants.rs
 * - 定数置き場
 */


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