
use crate::data::enums::{
	Koma,
	Side
};
use crate::data::types::{
	Cell,
};
// use crate::data::board::{
// 	Board,
// };

// Sideの表示用実装
impl Side {
	pub fn render(&self) -> &str {
		match self {
			Side::A => "A",
			Side::B => "B",
			_ => "Free"
		}
	}
}


impl Koma {
	pub fn render(&self) -> char {
		match self {
			Koma::Lion =>'🦁',
			Koma::Hiyoko => '🐥',
			Koma::Kirin => '🦒',
			Koma::Zou => '🐘',
			Koma::Niwatori => '🐔',
			_ => '　'
		}
	}
}


// セルの表示用impl
impl Cell {
	pub fn render_koma(&self) -> char {
		self.koma.render()
	}
	pub fn render_side(&self) -> char {
		match self.side {
			Side::A =>'Ａ',
			Side::B => 'Ｂ',
			_ => '　'
		}
	}
	pub fn render(&self) -> String {
		return format!("{}{}", self.render_koma(), self.render_side());
	}
}
