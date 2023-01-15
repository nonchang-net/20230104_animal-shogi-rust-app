/**
 * Board.rs
 * - 盤面状態と手駒状態のセット
 */

use std::collections::HashMap;
use std::cell::RefCell;

use crate::data::types::{
	Cell,
};

use super::{enums::{Koma, Side}, constants::INITIAL_BOARD_DATA, types::Position};

// type Tegomas = HashMap<Side, Vec<Koma>>;

#[allow(dead_code, unused_variables)]
#[derive(Debug, Clone)]
pub struct Board{
	// 盤情報の二次元配列
	pub data: [[Cell; 3]; 4],

	// 手駒: sideをキーにしたKomaの配列
	pub tegomas: RefCell<HashMap<Side, Vec<Koma>>>,

	// イテレータの現在処理位置
	// TODO: イテレータ実装は別structにしてBoard自体から省きたいかな。評価時にcloneする意味がない
	iter_x: usize,
	iter_y: usize,

	// side側の「効いてる場所」の一覧
	pub attackable_map: RefCell<HashMap<Side, FlagBoard>>,

	// sideがチェックメイトされているかどうか
	pub is_checkmate_map: RefCell<HashMap<Side, bool>>,
}

#[allow(dead_code)]
impl Board{

	pub fn new() -> Self {
		let mut _board = Self{
			data: INITIAL_BOARD_DATA,
			tegomas: RefCell::new(HashMap::new()),
			iter_x: 0,
			iter_y: 0,
			// is_pre_evaluated: false,
			attackable_map: RefCell::new(HashMap::new()),
			is_checkmate_map: RefCell::new(HashMap::new()),
		};
		return _board;
	}

	pub fn get_or_create_attackable_map(&self, side:&Side) -> FlagBoard{
		// arrackable_mapsから取得できなければ生成、あれば取得してclone()をreturnする
		// undone: clone()するのが気になるものの、まずは生成済なら再計算しない、と言うことが主眼なので一旦これで。。borrow checkerとの戦いに疲れ切っているorz
		let mut map = self.attackable_map.borrow_mut();
		let result = map.get(&side);
		match result {
			Some(flag_board) => {
				// println!("DEBUG: get_or_create_attackable_map() get cached.");
				flag_board.clone()
			},
			_ => {
				// println!("DEBUG: get_or_create_attackable_map() new created.");
				let new_result = self.create_attackable_map(&side);
				map.insert(*side, new_result.clone());
				return new_result;
			}
		}
	}

	fn create_attackable_map(&self, side:&Side) -> FlagBoard{
		// side側のattackable_mapを新規作成
		let mut attackable_map = FlagBoard::new(false);
		for x in 0..3{
			for y in 0..4{
				let cell = self.data[y][x];
				// 自陣の駒以外のセルは評価しない
				if cell.side != *side {
					continue;
				}
				let rules = cell.koma.get_move_rule_from_side_a();
				let pos = Position {
					x:x as i8,
					y:y as i8
				};
				for rule in rules {
					let target_pos = pos.add(&rule, side);
					if !target_pos.is_valid() { continue; }

					// 自駒が攻撃できる場所。trueにする
					// println!("DEBUG: target_pos:{:?}", target_pos);
					attackable_map.data[target_pos.y as usize][target_pos.x as usize] = true;
				}
			}
		}
		return attackable_map
	}

	// sideがチェックメイトされているか確認
	// 計算済みならキャッシュから返す
	pub fn get_or_create_is_checkmate(&self, side:&Side) -> bool {
		// すでに計算済みかどうか確認
		let mut map = self.is_checkmate_map.borrow_mut();
		let result = map.get(&side);
		match result {
			Some(x) => {
				// println!("DEBUG: get_or_create_is_checkmate() get cached.");
				return *x
			},
			_ => {
				// キーが存在しないので新規作成
				// println!("DEBUG: get_or_create_is_checkmate() new created.");
				let new_result = self.create_is_checkmate(side);
				map.insert(*side, new_result);
				return new_result;
				return false;
			}
		}
	}

	// sideがチェックメイトされているか確認
	fn create_is_checkmate(&self, side:&Side) -> bool {
		let lion_pos = self.search_lion_pos(side);
		// 相手側のattackable_mapを取得
		let flag_board = self.get_or_create_attackable_map(&side.reverse());
		return flag_board.data[lion_pos.y as usize][lion_pos.x as usize];
	}

	// ライオンの位置を取得
	// - 盤面評価でライオンが不在な状況で盤面を評価してはいけないので、見つからないときはpanicで終了
	fn search_lion_pos(&self, side:&Side) -> Position {
		let result = self.search_koma_pos(&side, &Koma::Lion);
		match result {
			Some(x) => x,
			_ => {
				// TODO: どっちサイドでpanicしたか出力したいものの、dataクラスはview処理を使いたくないので悩み中。一旦side情報なしでpanicしておく
				// let str = format!("検索したSideにKoma::Lionが見つかりませんでした。ゲームオーバー状態のBoardは評価できません。", koma.render());
				panic!("検索したSideにKoma::Lionが見つかりませんでした。ゲームオーバー状態のBoardは評価できません。");
			}
		}
	}

	// sideのkomaを検索、最初に見つかったものを返す
	// - lion検索以外に使ってないけど一応括り出しておく
	fn search_koma_pos(&self, side:&Side, koma:&Koma) -> Option<Position> {
		for x in 0..3{
			for y in 0..4{
				let cell = self.data[y][x];
				if cell.side == *side && cell.koma == *koma {
					return Some(Position{x:x as i8,y:y as i8});
				}
			}
		}
		None
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
