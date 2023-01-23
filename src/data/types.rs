/**
 * 細かい型やstruct定義の一次置き場
 * - 実装などが入ったものは順次独立したファイルにしていく
 */

use crate::data::enums::{
	Koma,
	Side
};

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
// Board上の座標を示すタプル
// - 「if i<0」判定をする可能性を鑑みて符号付き
pub struct Position {
	pub x:i8,
	pub y:i8
}

impl Position {
	// position同士の加算
	// - sideごとの移動ルールを適用する際に利用
	pub fn add(&self, pos:&Position, side:&Side) -> Position{
		let is_side_a = side == &Side::A;
		let new_position = Position{
			x: self.x + pos.x,
			y: self.y + if is_side_a { pos.y } else { pos.y * -1 }
		};
		return new_position;
	}

	// 盤の範囲内かどうか
	pub fn is_valid(&self) -> bool {
		// note: ショートサーキットを期待して、失敗系のorを最終的に反転
		return !(self.x < 0 || self.x >= 3 || self.y < 0 || self.y >= 4);
	}
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
// 移動手を示す構造体
pub struct Move{
	pub from: Position,
	pub to: Position
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
// コマ配置手を示す構造体
pub struct Put{
	pub index: i8,
	pub to: Position
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
// 1手を示す構造体
// - TODO: 必ずmove_handかput_handが存在する、と言う定義にしたいけど無理そう？
pub struct Hand{
	pub move_hand: Option<Move>,
	pub put_hand: Option<Put>
}

// 盤上の1セルの状態
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Cell{
	pub side: Side,
	pub koma: Koma,
}




// ******  ********    ******  ******  
//   **    **        **          **    
//   **    ********    ****      **    
//   **    **              **    **    
//   **    ********  ******      **    

#[cfg(test)]
mod types_tests {
	use crate::data::constants::*;
	use super::*;

	#[test]
	fn test_position_side_add() {
		// test: position.add()のsideごとの挙動を確認する
		let position_a = Position {y: 1, x: 1};
		// ひよこの移動手を試す
		let move_position = KOMA_MOVERULE_HIYOKO[0];
		// test: Side::Aなら上に行く
		assert_eq!(position_a.add(&move_position, &Side::A), Position {y:0, x:1});
		// test: Side::Bなら下に行く
		assert_eq!(position_a.add(&move_position, &Side::B), Position {y:2, x:1});
	}

	#[test]
	fn test_position_is_valid() {
		// 盤上判定が正しいかチェック
		assert_eq!(Position {y:0, x:0}.is_valid(), true);
		assert_eq!(Position {y:3, x:2}.is_valid(), true);

		assert_eq!(Position {y: 0,x:-1}.is_valid(), false);
		assert_eq!(Position {y:-1,x: 0}.is_valid(), false);
		assert_eq!(Position {y: 0,x: 3}.is_valid(), false);
		assert_eq!(Position {y: 4,x: 0}.is_valid(), false);
	}

}