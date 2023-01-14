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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Board{
	// 盤情報の二次元配列
	pub data: [[Cell; 3]; 4],

	// 手駒: sideをキーにしたKomaの配列
	pub tegomas: RefCell<HashMap<Side, Vec<Koma>>>,

	// イテレータの現在処理位置
	iter_x: usize,
	iter_y: usize,

	// 効いてる場所の一覧
	pub attackable_maps: RefCell<HashMap<Side, FlagBoard>>,
}

#[allow(dead_code)]
impl Board{

	pub fn new() -> Self {
		let mut _board = Self{
			data: INITIAL_BOARD_DATA,
			tegomas: RefCell::new(HashMap::new()),
			iter_x: 0,
			iter_y: 0,
			attackable_maps: RefCell::new(HashMap::new()),
		};
		return _board;
	}

	fn test(&self) -> u32 {
		// self.width * self.height
		return 123;
	}

	pub fn get_or_create_attackable_map(&mut self, side:&Side) -> FlagBoard{
		// arrackable_mapsから取得できなければ生成、あれば取得してclone()をreturnする
		// undone: clone()するのが気になるものの、まずは生成済なら再計算しない、と言うことが主眼なので一旦これで。。borrow checkerとの戦いに疲れ切っているorz
		let mut maps = self.attackable_maps.borrow_mut();
		let result = maps.get(&side);
		match result {
			Some(flag_board) => flag_board.clone(),
			_ => {
				let new_result = self.create_attackable_map(&side);
				maps.insert(*side, new_result.clone());
				return new_result;
			}
		}
	}

	fn create_attackable_map(&self, side:&Side) -> FlagBoard{
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

/*
	public GetAttackableMaps():[BoolMap, BoolMap]{
		var sideAMap = Utils.GetFilledFlagBoard(false)
		var sideBMap = Utils.GetFilledFlagBoard(false)
		this.Each((pos)=>{
			var cell=this.Get(pos)
			if(cell.side === Side.Free) return;
			const moveRules = Utils.GetKomaMoveRules(cell.koma)
			for(const rulePos of moveRules){
				// rulePosを適用した移動先セルを取得
				const targetPos = pos.Add(rulePos, cell.side)
	
				// 盤の範囲外は除外
				if(!targetPos.IsValidIndex()) continue;
	
				if(cell.side === Side.A){
					sideAMap[targetPos.y][targetPos.x] = true;
				}else if(cell.side === Side.B){
					sideBMap[targetPos.y][targetPos.x] = true;
				}else{
					throw new Error(`undefined index. ${cell.side}`)
				}
			}
		})
		return [sideAMap, sideBMap]
	}

*/




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
