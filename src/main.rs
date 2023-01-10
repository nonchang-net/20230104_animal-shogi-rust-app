use std::collections::HashMap;

// #![allow(unused)]

mod data;

use crate::data::enums::{
	Koma,
	Side
};
use crate::data::types::{
	Board,
	Cell,
};


fn main() {

	// Board構造体の初期化テスト
	let _board = Board{
		data: [
			[
				Cell{side: Side::A, koma:Koma::Null},
				Cell{side: Side::A, koma:Koma::Null},
				Cell{side: Side::A, koma:Koma::Null}
			],
			[
				Cell{side: Side::A, koma:Koma::Null},
				Cell{side: Side::A, koma:Koma::Null},
				Cell{side: Side::A, koma:Koma::Null}
			],
			[
				Cell{side: Side::A, koma:Koma::Null},
				Cell{side: Side::A, koma:Koma::Null},
				Cell{side: Side::A, koma:Koma::Null}
			],
			[
				Cell{side: Side::A, koma:Koma::Null},
				Cell{side: Side::A, koma:Koma::Null},
				Cell{side: Side::A, koma:Koma::Null}
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
	// println!("{:?}", Side::A.reverse());

	render();

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
fn render() {
	println!();
	println!("animal shogi: ver20230104.2006");
	println!();
	println!("  : ａ　ｂ　ｃ　: ----------------");
	println!("==:============ : Side.B captured:");
	println!(" 1:🐘Ｂ🦁Ｂ🦒Ｂ : none");
	println!(" 2:　　🐥Ｂ　　 : ----------------");
	println!(" 3:　　🐥Ａ　　 : Side.A captured:");
	println!(" 4:🦒Ａ🦁Ａ🐘Ａ : none");
	println!();
	println!("Side.A's turn. YOU ARE CHECKMATED!!!");
	println!("command: (? to show help. q to quit)");
}