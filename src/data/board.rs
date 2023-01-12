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

	// 手駒: sideをキーにしたKomaの配列
	pub tegomas: HashMap<Side, Vec<Koma>>,

	// 効いてる場所の一覧
	// TODO: なんかここでエラー出ている
	// pub arrackable_maps: HashMap<Side, FlagBoard>,
}

#[allow(dead_code)]
impl Board{
	fn test(&self) -> u32 {
		// self.width * self.height
		return 123;
	}

	fn get_attackable_map(side:Side) -> FlagBoard{
		// TODO: arrackable_mapsから取得できなければ生成、あれば取得してreturnする
		let mut attackable_map = FlagBoard::new(false);
		return attackable_map
	}
}

// 盤状態のフラグマップを表現する型
#[derive(Debug, Clone)]
pub struct FlagBoard{
    pub data: [[bool; 3] ;4]
}

#[allow(dead_code)]
impl FlagBoard{
    fn new(flag:bool) -> FlagBoard{
		return FlagBoard {
			data: [
				[flag,flag,flag],
				[flag,flag,flag],
				[flag,flag,flag],
				[flag,flag,flag]
			]
		};
    }
}
