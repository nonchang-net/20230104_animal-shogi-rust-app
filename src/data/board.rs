/**
 * Board.rs
 * - 盤面状態と手駒状態のセット
 */

use std::collections::HashMap;

use crate::data::types::{
	Cell,
};

use super::{enums::{Koma, Side}, constants::INITIAL_BOARD_DATA};

// type Tegomas = HashMap<Side, Vec<Koma>>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Board{
	// 盤情報の二次元配列
	pub data: [[Cell; 3]; 4],

	// 手駒: sideをキーにしたKomaの配列
	pub tegomas: HashMap<Side, Vec<Koma>>,

	// イテレータの現在処理位置
	iter_x: usize,
	iter_y: usize,

	// 効いてる場所の一覧
	// TODO: なんかここでエラー出ている
	// pub arrackable_maps: HashMap<Side, FlagBoard>,
}

#[allow(dead_code)]
impl Board{

	pub fn new() -> Self {
		let _board = Self{
			data: INITIAL_BOARD_DATA,
			tegomas: HashMap::new(),
			iter_x: 0,
			iter_y: 0,
		};
		return _board;
	}

	fn test(&self) -> u32 {
		// self.width * self.height
		return 123;
	}

	pub fn get_attackable_map(side:Side) -> FlagBoard{
		// TODO: arrackable_mapsから取得できなければ生成、あれば取得してreturnする
		let mut attackable_map = FlagBoard::new(false);
		return attackable_map
	}
}

// Boardの全てのCellを順に返すイテレータ
impl Iterator for Board{
	type Item = Cell;
	fn next(&mut self) -> Option<Self::Item> {
		self.iter_x += 1;
		if self.iter_x >= 3 {
			self.iter_x = 0;
			self.iter_y += 1;
			if self.iter_y >=4 {
				// 終了している
				return None;
			}
		}
		return Some(self.data[self.iter_y][self.iter_x]);
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		return (12, Some(12));
	}
}

impl ExactSizeIterator for Board{
	fn len(&self) -> usize { return 12; }
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
