// use std::str;
use std::collections::HashMap;

// #![allow(unused)]

mod data;

use crate::data::enums::{
	Koma,
	Side
};
use crate::data::types::{
	Cell,
};
use crate::data::board::{
	Board,
};


fn main() {

	// Board構造体の初期化テスト
	let _board = Board{
		data: [
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
		],
		tegomas: HashMap::new()
	};

	// Cell構造体の初期化テスト
	let _cell = Cell{
		side: Side::A,
		koma: Koma::Hiyoko,
	};

	// デバッグ出力テスト
	// dbg!(_board);
	// println!("{:?}", _board);
	// println!("{:?}", _cell);
	// println!("side:reverse(): {:?}", Side::A.reverse());

	// 盤面テスト
	println!("{}",_board.render());

	// 入力ループテスト
	// - TODO: 当面不要なので保留
	// loop{
	//     let answer = get_input();
	//     println!("{}", answer);
	//     if answer == "q" { break; }
	// }
}



#[allow(dead_code)]
fn get_input() -> String {
	let mut word = String::new();
	std::io::stdin().read_line(&mut word).ok();
	return word.trim().to_string();
}


// Boardの表示用impl
impl Board {
	pub fn render(&self) -> String {
		let mut result = String::new();

		// ヘッダーとstatus枠表示
		result.push_str("  : ａ　ｂ　ｃ　: Side.B captured\n");
		result.push_str("==:============ : ");
		result.push_str(self.render_motigoma(Side::B).as_str());
		result.push('\n');

		// セル表示開始
		for (index, line) in self.data.iter().enumerate() {
			result.push_str(format!(" {}:", index+1).as_str());
			for cell in line.iter() {
				result.push_str(cell.render().as_str())
			}
			// ステータス枠表示
			match index {
				0 => result.push_str(" : Side.A captured\n"),
				1 => {
					result.push_str(" : ");
					result.push_str(self.render_motigoma(Side::A).as_str());
					result.push('\n');
				},
				_ => result.push_str(" :\n")
			}
		}
		return result;
	}
	
	pub fn render_motigoma(&self, side:Side) -> String {
		let mut result = String::new();
		let komalist = self.tegomas.get(&side);
		match komalist {
			Some(x) => for koma in x {
				result.push(koma.render())
			},
			None => result.push_str("none")
		}
		return result;
	}

	// pub fn test2() -> &'static str{
	// 	"test"
	// }

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
