// use std::str;
use std::collections::HashMap;

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

mod view;



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
	println!("");
	println!("{}",_board.render());
	println!("{}",_board.render_infomation(Side::A));
	

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
