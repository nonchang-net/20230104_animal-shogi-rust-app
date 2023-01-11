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
				Cell{side: Side::B, koma:Koma::Kirin},
				Cell{side: Side::B, koma:Koma::Lion},
				Cell{side: Side::B, koma:Koma::Zou}
			],
			[
				Cell{side: Side::Free, koma:Koma::Null},
				Cell{side: Side::B, koma:Koma::Hiyoko},
				Cell{side: Side::Free, koma:Koma::Null}
			],
			[
				Cell{side: Side::Free, koma:Koma::Null},
				Cell{side: Side::A, koma:Koma::Hiyoko},
				Cell{side: Side::Free, koma:Koma::Null}
			],
			[
				Cell{side: Side::A, koma:Koma::Zou},
				Cell{side: Side::A, koma:Koma::Lion},
				Cell{side: Side::A, koma:Koma::Kirin}
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
	println!("side:reverse(): {:?}", Side::A.reverse());

	// 盤面テスト
	render(&_board);

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


#[allow(dead_code)]
fn render(board:&Board) {
	// println!();
	// println!("animal shogi: ver20230104.2006");
	// println!();
	// println!("  : ａ　ｂ　ｃ　: ----------------");
	// println!("==:============ : Side.B captured:");
	// println!(" 1:🐘Ｂ🦁Ｂ🦒Ｂ : none");
	// println!(" 2:　　🐥Ｂ　　 : ----------------");
	// println!(" 3:　　🐥Ａ　　 : Side.A captured:");
	// println!(" 4:🦒Ａ🦁Ａ🐘Ａ : none");
	// println!();
	// println!("Side.A's turn. YOU ARE CHECKMATED!!!");
	// println!("command: (? to show help. q to quit)");

	println!(" 1:{}{}{} :",
		board.data[0][0].render(),
		board.data[0][1].render(),
		board.data[0][2].render()
	);
}

// セルの表示用impl
impl Cell {
	pub fn render_koma(&self) -> char {
		match self.koma {
			Koma::Lion =>'🦁',
			Koma::Hiyoko => '🐥',
			Koma::Kirin => '🦒',
			Koma::Zou => '🐘',
			Koma::Niwatori => '🐔',
			_ => '　'
		}
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
