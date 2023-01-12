/**
 * 細かい型やstruct定義の一次置き場
 * - 実装などが入ったものは順次独立したファイルにしていく
 */

use crate::data::enums::{
	Koma,
	Side
};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
// Board上の座標を示すタプル
// - 「if i<0」判定をする可能性を鑑みて符号付き
pub struct Position (i8, i8);

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
pub struct Hand{
	pub move_hand: Move,
	pub put: Put
}

// 盤上の1セルの状態
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Cell{
	pub side: Side,
	pub koma: Koma,
}



