/**
 * 細かい型やstruct定義の一次置き場
 * - 実装などが入ったものは順次独立したファイルにしていく
 */

use crate::data::enums::{
	Koma,
	Side
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cell{
	pub side: Side,
	pub koma: Koma,
}



