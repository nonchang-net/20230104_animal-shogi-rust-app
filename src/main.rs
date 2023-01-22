use data::enums::SideState;
use rand::prelude::*;

mod data;
mod view;

use crate::{
	data::{
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

	// let mut _board = Board::new();

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
	// println!("");
	// println!("{}",_board.render());
	// println!("{}",_board.render_infomation(&Side::A));

	// Board Iteratorのテスト
	// TODO: _board.iter_all_cells()って書くにはどう実装したらいいのかな
	// for x in _board {
	// 	println!("Cell: {:?}", x)
	// }

	// DEBUG: attackable map取得テスト
	// let attackable_map = _board.get_or_create_attackable_map(&Side::A);
	// println!("attackable_map: {:?}", attackable_map.data);
	// let attackable_map_a2 = _board.get_or_create_attackable_map(&Side::A);
	// println!("attackable_map: {:?}", attackable_map_a2.data);
	// let attackable_map_b = _board.get_or_create_attackable_map(&Side::B);
	// println!("attackable_map: {:?}", attackable_map_b.data);

	let mut _game = Game::new();
	_game.start();
	
}




// ゲーム挙動実装とコンピューター判断のテスト用struct/impl
struct Game{
	board: Board,
	current_side: Side,
	current_turn: u32,
	rng: ThreadRng,
}

#[allow(dead_code)]
impl Game{
	pub fn new() -> Self{
		let _game = Self{
			board: Board::new(),
			current_side: Side::A,
			current_turn: 1,
			rng: rand::prelude::thread_rng(),
		};
		return _game;
	}

	pub fn start(&mut self) {
		// ゲームループ開始
		loop{
			self.show();
			// 入力
			let answer = Self::get_input();
			if answer == "q" { break; }

			// ゲームオーバー評価
			// - evaluate_gamestate()して負けてたらゲームオーバーにする
			self.board.evaluate_gamestate();
			let side_idx = if self.current_side == Side::A { 0 } else { 1 };
			match self.board.states[side_idx] {
				None => {
					panic!("stateが初期化されていません。")
				},
				Some(x) => {
					match x {
						SideState::Playable =>{
							// 次ターン評価
							self.next();
						},
						_ => {
							println!("GAME OVER!!!!!!");
							break;
						}
					}
				}
			}
		}
	}

	// 相手ターンにして一手進める
	pub fn next(&mut self) {

		// ランダムな手を一つ選択する
		let hands = self.board.get_or_create_valid_hands(&self.current_side);
		let index = self.rng.gen_range(0, hands.len());

		// ランダムに打つ
		// self.board = self.board.get_hand_applied_clone(&self.current_side, &hands[index]);
		// TEST: 実行時panicが出ているので、試しにboard.clone()してからさらにapplied_clone()を取得してみる……。
		// → 何度か試したらやはり同じところでエラー。
		self.board = self.board.clone().get_hand_applied_clone(&self.current_side, &hands[index]);

		// 次のターンに変更する
		self.current_turn += 1;
		self.current_side = self.current_side.reverse();
	}

	// 現在の情報を表示する
	fn show(&mut self) {
		println!("{}",self.board.render());
		println!("{}",self.board.render_infomation(&self.current_side));

		println!("current turn: {}", self.current_turn);
	}

	// CUIから一列取得
	fn get_input() -> String {
		let mut word = String::new();
		std::io::stdin().read_line(&mut word).ok();
		return word.trim().to_string();
	}
}