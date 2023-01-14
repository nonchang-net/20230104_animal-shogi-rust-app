// use std::str;
use std::collections::HashMap;

mod data;
mod view;

use crate::{
	data::{
		constants::INITIAL_BOARD_DATA,
		enums::{
			// Koma,
			Side
		},
		// types::{
		// 	Cell,
		// },
		board::{
			Board,
		},
	},
};



fn main() {

	let _board = Board::new();

	// Cell構造体の初期化テスト
	// let _cell = Cell{
	// 	side: Side::A,
	// 	koma: Koma::Hiyoko,
	// };

	// デバッグ出力テスト
	// dbg!(_board);
	// println!("{:?}", _board);
	// println!("{:?}", _cell);
	// println!("side:reverse(): {:?}", Side::A.reverse());

	// 盤面テスト
	println!("");
	println!("{}",_board.render());
	println!("{}",_board.render_infomation(Side::A));

	// Board Iteratorのテスト
	// TODO: _board.iter_all_cells()って書くにはどう実装したらいいのかな
	// for x in _board {
	// 	println!("Cell: {:?}", x)
	// }

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
