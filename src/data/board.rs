/**
 * Board.rs
 * - 盤面状態と手駒状態のセット
 */

use std::collections::HashMap;

use crate::data::types::{
	Cell,
};

use super::enums::{Koma, Side};

// type Tegomas = HashMap<Side, Vec<Koma>>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Board{
	// 盤情報の二次元配列
	pub data: [[Cell; 3]; 4],

	// それぞれのサイドの手駒sideをキーにしたKomaの配列
	pub tegomas: HashMap<Side, Vec<Koma>>
}

#[allow(dead_code)]
impl Board{
	fn test(&self) -> u32 {
		// self.width * self.height
		return 123;
	}
}

// 盤状態のフラグマップを表現する型
#[derive(Debug)]
pub struct FlagBoard{
    pub data: [[bool; 3] ;4]
}

#[allow(dead_code)]
impl FlagBoard{
    fn test(&self) -> u32{
        return 234;
    }
}
