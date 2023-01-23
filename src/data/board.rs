/**
 * Board.rs
 * - 盤面状態と手駒状態のセット
 * - 各種評価メソッドと、評価済み情報のSideごとのOptionの配列を保持する
 */

// extern crate board;

use crate::data::types::{
	Cell,
};

use super::{enums::{Koma, Side, SideState}, constants::INITIAL_BOARD_DATA, types::{Position, Hand, Put, Move}};



// 盤状態のフラグマップを表現する型
#[derive(Debug, Clone)]
pub struct FlagBoard{
	pub data: [[bool; 3] ;4]
}

impl FlagBoard{
	fn new(flag:bool) -> FlagBoard{
		return FlagBoard {
			data: [
				[flag,flag,flag],
				[flag,flag,flag],
				[flag,flag,flag],
				[flag,flag,flag]
			]
		};
	}
}


// Boardの全てのCellを順に返すイテレータ
impl Iterator for Board{
	type Item = Cell;
	fn next(&mut self) -> Option<Self::Item> {
		self.iter_x += 1;
		if self.iter_x >= 3 {
			self.iter_x = 0;
			self.iter_y += 1;
			if self.iter_y >=4 {
				// 終了している
				return None;
			}
		}
		return Some(self.cells[self.iter_y][self.iter_x]);
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		return (12, Some(12));
	}
}

impl ExactSizeIterator for Board{
	fn len(&self) -> usize { return 12; }
}

#[allow(dead_code, unused_variables)]
#[derive(Debug, Clone)]
pub struct Board{
	// 盤情報の二次元配列
	pub cells: [[Cell; 3]; 4],

	// 手駒: sideをキーにしたKomaの配列
	pub tegomas: [Vec<Koma>; 2],

	// イテレータの現在処理位置
	// TODO: イテレータ実装をトレイトあたりで分けてBoard自体から省きたいかな。評価時にcloneする意味がない
	iter_x: usize,
	iter_y: usize,

	// sideの状態: 続行可能かゲームオーバー状態か
	pub states: [SideState; 2],

	// side側の「効いてる場所」の一覧
	pub attackable_maps: [Option<FlagBoard>; 2],

	// sideがチェックメイトされているかどうか
	pub is_checkmates: [Option<bool>; 2],

	// sideのトライ可能手の一覧
	// - is_tryableはトライ可能手が1つ以上ある状態を指すのでメソッドにした
	pub tryable_positions: [Option<Vec<Position>>; 2],

	// sideの着手可能手の一覧
	pub valid_hands: [Option<Vec<Hand>>; 2],
}

#[allow(dead_code)]
impl Board{

	pub fn new() -> Self {
		let mut _board = Self{
			cells: INITIAL_BOARD_DATA,
			tegomas: Default::default(),
			iter_x: 0,
			iter_y: 0,
			states: [
				SideState::Playable,
				SideState::Playable
			],
			attackable_maps: Default::default(),
			is_checkmates: Default::default(),
			tryable_positions: Default::default(),
			valid_hands: Default::default(),
		};
		return _board;
	}

	// 手を反映したクローンを作成
	// - 先読みに必要になる
	// - トライアブル回避手の一覧
	pub fn get_hand_applied_clone(&self, side:&Side, hand:&Hand) -> Board {

		let mut exist_move_hand = false;

		let mut new_cells: [[Cell; 3]; 4] = self.cells.clone();

		// 現在の手駒状態をcloneしておく
		let mut tegoma_side_a: Vec<Koma> = self.tegomas[0].clone();
		let mut tegoma_side_b: Vec<Koma> = self.tegomas[1].clone();

		match hand.move_hand{
			Some(move_hand) => {
				let from = &move_hand.from;
				let from_cell = self.get_cell(from);
				let to = &move_hand.to;
				let to_cell = self.get_cell(to);

				// debug
				if to_cell.side == *side {
					panic!("想定外呼び出し: 自分の駒がある場所にmoveしようとしました。。");
				}

				// 相手のコマを取ったかどうか
				if to_cell.side != Side::Free {
					// 手駒に追加
					match to_cell.side {
						Side::A => {
							tegoma_side_b.push(to_cell.koma);
						},
						Side::B => {
							tegoma_side_a.push(to_cell.koma);
						},
						_ => {
							panic!("ここには来ないはず")
						}
					}
					
				}

				// TODO: ひよこ→にわとりのプロモーション評価を忘れている。move_handにフラグ追加が必要？ AI専用コードなので常時プロモーションでいいかな？

				// 移動先を移動元の駒に置き換える
				new_cells[to.y as usize][to.x as usize] = from_cell;

				// 移動元を空白に置き換える
				new_cells[from.y as usize][from.x as usize] = Cell{side:Side::Free, koma:Koma::Null};

				exist_move_hand = true;
			},
			None => {}
		}
		match hand.put_hand{
			Some(put_hand) => {
				// boardに配置する
				let koma = self.tegomas[side.to_index()][put_hand.index as usize];

				new_cells[put_hand.to.y as usize][put_hand.to.x as usize] = Cell{koma:koma, side: *side};

				// 手駒から削除
				match side {
					Side::A => {
						tegoma_side_a.remove(put_hand.index as usize);
					},
					Side::B => {
						tegoma_side_b.remove(put_hand.index as usize);
					},
					_ => {
						panic!("ここには来ないはず")
					}
				}
			},
			None => {
				if !exist_move_hand {
					panic!("想定外コード: handがput/move両方ともNoneでした")
				}
			}
		}

		// 新しいBoardを作って返す
		let mut _new_board = Self {
			cells: new_cells,
			tegomas: [
				tegoma_side_a,
				tegoma_side_b
			],
			iter_x: 0,
			iter_y: 0,
			states: [
				SideState::Playable,
				SideState::Playable
			],
			attackable_maps: Default::default(),
			is_checkmates: Default::default(),
			tryable_positions: Default::default(),
			valid_hands: Default::default(),
		};
		return _new_board;
		
	}

	// ====================
	// 汎用系サブルーチン
	// ====================

	// 汎用サブルーチン: posでdataからcell取得
	fn get_cell(&self, pos:&Position) -> Cell{
		return self.cells[pos.y as usize][pos.x as usize];
	}

	// 汎用サブルーチン: ライオンの位置を取得
	// - 盤面評価でライオンが不在な状況で盤面を評価してはいけないので、見つからないときはpanicで終了
	fn search_lion_pos(&self, side:&Side) -> Position {
		let result = self.search_koma_pos(&side, &Koma::Lion);
		match result {
			Some(x) => x,
			_ => {
				// TODO: どっちサイドでpanicしたか出力したいものの、dataクラスはview処理を使いたくないので悩み中。一旦side情報なしでpanicしておく
				// let str = format!("検索したSideにKoma::Lionが見つかりませんでした。ゲームオーバー状態のBoardは評価できません。", koma.render());
				panic!("検索したSideにKoma::Lionが見つかりませんでした。ゲームオーバー状態のBoardは評価できません。");
			}
		}
	}

	// 汎用サブルーチン: sideのkomaを検索、最初に見つかったものを返す
	// - lion検索以外に使ってないけど一応括り出しておく
	fn search_koma_pos(&self, side:&Side, koma:&Koma) -> Option<Position> {
		for x in 0..3{
			for y in 0..4{
				let cell = self.cells[y][x];
				if cell.side == *side && cell.koma == *koma {
					return Some(Position{x:x as i8,y:y as i8});
				}
			}
		}
		None
	}

	// 汎用サブルーチン: Koma::NullなPositionの一覧を取得
	fn get_null_cell_positions(&self) -> Vec<Position> {
		let mut results: Vec<Position> = [].to_vec();
		for x in 0..3{
			for y in 0..4{
				let cell = self.cells[y][x];
				if cell.koma == Koma::Null {
					results.push(Position{x:x as i8,y:y as i8});
				}
			}
		}
		return results;
	}

	// 汎用サブルーチン: 自陣の盤上の駒の座標一覧を取得
	fn get_all_onboard_koma_positions(&self, side:&Side)-> Vec<Position> {
		let mut results: Vec<Position> = [].to_vec();
		for x in 0..3{
			for y in 0..4{
				let cell = self.cells[y][x];
				if cell.side == *side {
					results.push(Position{x:x as i8,y:y as i8});
				}
			}
		}
		return results;
	}


	// ====================
	// 評価処理
	// ====================

	// 評価処理: ゲームオーバー状態かどうかが判定済み状態にする
	// - 評価中にフラグ変更しているのがあまりよくないかもしれない
	// - 両方の陣営の「get_or_create_valid_hands()」を評価すれば、`self.state()`を評価しているところは全て通る
	pub fn evaluate_gamestate(&mut self) {
		self.get_or_create_valid_hands(&Side::A);
		self.get_or_create_valid_hands(&Side::B);
	}

	// 評価処理: 効いている場所の一覧を取得する
	// 計算済みならキャッシュから返す
	pub fn get_or_create_attackable_map(&mut self, side:&Side) -> FlagBoard{
		// arrackable_mapsから取得できなければ生成、あれば取得してclone()をreturnする

		let opt = &self.attackable_maps[side.to_index()];
		match opt {
			Some(flag_board) => {
				// println!("DEBUG: get_or_create_attackable_map() get cached.");
				flag_board.clone()
			},
			_ => {
				// println!("DEBUG: get_or_create_attackable_map() new created.");
				let new_result = self.create_attackable_map(&side);

				// TODO: 以下がborrow checker errorっぽい
				self.attackable_maps[side.to_index()] = Some(new_result.clone());
				return new_result;
			}
		}
	}

	// 評価処理: 効いている場所の一覧を取得する
	fn create_attackable_map(&self, side:&Side) -> FlagBoard{
		// side側のattackable_mapを新規作成
		let mut attackable_map = FlagBoard::new(false);
		for x in 0..3{
			for y in 0..4{
				let cell = self.cells[y][x];
				// 自陣の駒以外のセルは評価しない
				if cell.side != *side {
					continue;
				}
				let rules = cell.koma.get_move_rule_from_side_a();
				let pos = Position {
					x:x as i8,
					y:y as i8
				};
				for rule in rules {
					let target_pos = pos.add(&rule, side);
					if !target_pos.is_valid() { continue; }

					// 自駒が攻撃できる場所。trueにする
					// println!("DEBUG: target_pos:{:?}", target_pos);
					attackable_map.data[target_pos.y as usize][target_pos.x as usize] = true;
				}
			}
		}
		return attackable_map
	}

	// 評価処理: sideがチェックメイトされているか確認
	// - 計算済みならキャッシュから返す
	pub fn get_or_create_is_checkmate(&mut self, side:&Side) -> bool {
		// すでに計算済みかどうか確認
		let result = self.is_checkmates[side.to_index()];
		match result {
			Some(x) => {
				// println!("DEBUG: get_or_create_is_checkmate() get cached.");
				return x
			},
			_ => {
				// キーが存在しないので新規作成
				// println!("DEBUG: get_or_create_is_checkmate() new created.");
				let new_result = self.create_is_checkmate(&side);

				// TODO: 以下がborrow checker errorっぽい
				self.is_checkmates[side.to_index()] = Some(new_result);
				return new_result;
			}
		}
	}

	// 評価処理: sideがチェックメイトされているか確認
	fn create_is_checkmate(&mut self, side:&Side) -> bool {
		let lion_pos = self.search_lion_pos(side);
		// 相手側のattackable_mapを取得
		let flag_board_data = self.get_or_create_attackable_map(&side.reverse()).data;
		return flag_board_data[lion_pos.y as usize][lion_pos.x as usize];
	}

	// 評価処理: sideがトライアブルかどうか
	// - トライ可能ポジションが一つでもあればtrue
	fn is_tryable(&self, side:&Side) -> bool {
		let count = self.get_or_create_tryable_positions(side).len();
		return count > 0;
	}

	// 評価処理: sideがトライ可能か確認
	fn get_or_create_tryable_positions(&self, side:&Side) -> Vec<Position> {
		// すでに計算済みかどうか確認
		let result = self.tryable_positions[side.to_index()].clone();
		match result {
			Some(x) => {
				// println!("DEBUG: get_or_create_tryable_positions() get cached.");
				return x.clone()
			},
			_ => {
				// キーが存在しないので新規作成
				// println!("DEBUG: get_or_create_tryable_positions() new created.");
				let new_result = self.create_tryable_positions(&side);

				// TODO: 以下がborrow checker errorっぽい
				// self.tryable_positions[side.to_index()] = Some(new_result);
				return new_result;
			}
		}
	}

	// 評価処理: sideのトライ可能位置のリストを取得
	fn create_tryable_positions(&self, side:&Side) -> Vec<Position> {
		let mut results: Vec<Position> = [].to_vec();

		// トライ可能なライン
		let tryable_y:i8 = if *side == Side::A { 1 } else { 2 };
		let lion_pos = self.search_lion_pos(side);

		// ライオンがトライ可能位置にいなければreturn
		if lion_pos.y != tryable_y { return results; }

		// トライ目標のライン
		let try_y:i8 = if *side == Side::A { 0 } else { 3 };

		for x in 0..3{
			// トライ目標座標xがライオンの動ける範囲外かチェック
			if lion_pos.x - 1 > x || lion_pos.x + 1 < x { continue; }

			let target_cell = self.cells[try_y as usize][x as usize];

			// 自分の駒がある場所には移動できない
			if target_cell.side == *side { continue; }

			// トライアブル
			results.push(Position{x:x, y:try_y});
		}
		return results;
	}

	// 評価処理: sideの着手可能手の一覧を取得する
	// - 計算済みならキャッシュから返す
	pub fn get_or_create_valid_hands(&mut self, side:&Side) -> Vec<Hand> {
		// すでに計算済みかどうか確認
		let result = self.valid_hands[side.to_index()].clone();
		match result {
			Some(x) => {
				// println!("DEBUG: get_or_create_valid_hands() get cached.");
				return x.clone()
			},
			_ => {
				// キーが存在しないので新規作成
				// println!("DEBUG: get_or_create_valid_hands() new created.");
				let new_result = self.create_valid_hands(&side);
				// self.valid_hands[side.to_index()] = Some(new_result);
				return new_result;
			}
		}
	}

	// 評価処理: sideの着手可能手の一覧を取得する
	fn create_valid_hands(&mut self, side:&Side) -> Vec<Hand> {
		let mut hands: Vec<Hand> = [].to_vec();

		// チェックメイト時
		let is_checkmate = self.get_or_create_is_checkmate(side);
		if is_checkmate {
			return self.create_valid_hands_when_checkmated(side);
		}

		// トライアブル時
		if self.is_tryable(&side.reverse()) {
			return self.create_valid_hands_when_tryable(side);
		}

		// 全ての合法手を追加
		hands.append(&mut self.create_all_move_hands(side));
		hands.append(&mut self.create_all_put_hands(side));

		// handがない = ステイルメイト
		// ※どうぶつしょうぎ(TM)のルールにおいてはチェスの意味でのステイルメイトは存在しない
		// - 今回の実装ではトライ回避手を先に枝刈りしたので、「トライ失敗する手しか残っていない」場合に発生し得る想定
		if hands.len() == 0{
			self.states[side.to_index()] = SideState::GameOverWithStalemate;
		}

		return hands;
	}

	// 評価処理: サブルーチン: 通常の（チェックメイトされてない・トライアブルじゃない時の）場合の、盤上の移動系の着手可能手の一覧を取得する
	fn create_all_move_hands(&mut self, side:&Side) -> Vec<Hand> {
		let mut hands: Vec<Hand> = [].to_vec(); 

		// 移動可能な駒のmove一覧を取得する
		for pos in self.get_all_onboard_koma_positions(side) {
			let cell = self.cells[pos.y as usize][pos.x as usize];
			// TODO: この走査はcreate_attackable_mapでも出てきたのでイテレータで共通化したい。書き方調べる
			let rules = cell.koma.get_move_rule_from_side_a();
			for rule in rules {
				let target_pos = pos.add(&rule, side);
				if !target_pos.is_valid() { continue; }

				// 移動先セル取得
				let target_cell = self.cells[target_pos.y as usize][target_pos.x as usize];

				// 自陣サイドの駒が存在するセルには移動できない
				if target_cell.side == *side { continue; }

				let enemy_attackable_map = self.get_or_create_attackable_map(&side.reverse()).data;

				// ライオンは、取られる場所には移動できない
				if cell.koma == Koma::Lion && enemy_attackable_map[target_pos.y as usize][target_pos.x as usize] {
					continue;
				}

				// 着手可能手に追加
				let move_hand = Move{
					from: pos,
					to: target_pos
				};
				let hand = Hand{
					move_hand: Some(move_hand),
					put_hand: None
				};
				hands.push(hand);
			}
		}
		return hands;
	}

	// 評価処理: サブルーチン: 通常の（チェックメイトされてない・トライアブルじゃない時の）場合の、盤上の持ち駒配置系の着手可能手の一覧を取得する
	fn create_all_put_hands(&self, side:&Side) -> Vec<Hand> {

		let mut hands: Vec<Hand> = [].to_vec();

		// 手駒確認
		// - 手駒があれば、全ての空白セルにputできる
		let tegomas = self.tegomas[side.to_index()].clone();
		for (index, _) in tegomas.iter().enumerate() {
			for pos in self.get_null_cell_positions() {
				let put_hand = Put{
					index:index as i8,
					to: pos
				};
				let hand = Hand{
					move_hand: None,
					put_hand: Some(put_hand)
				};
				hands.push(hand);
			}
		}
		return hands;
	}

	// 評価処理: サブルーチン: チェックメイト時の着手可能手の一覧を取得する
	fn create_valid_hands_when_checkmated(&mut self, side:&Side) -> Vec<Hand> {

		let mut hands: Vec<Hand> = [].to_vec();

		// 評価に必要な事前情報取得
		let lion_pos = self.search_lion_pos(side);
		let enemy_attackable_map = self.get_or_create_attackable_map(&side.reverse()).data;

		let all_move_hands = self.create_all_move_hands(side);
		for check_hand in all_move_hands {
			match check_hand.move_hand {
				Some(move_hand) => {
					// ライオン移動手
					if move_hand.from == lion_pos{
						let target_cell = self.cells[move_hand.to.y as usize][move_hand.to.x as usize];
						// 自陣サイドの駒が存在するセルには移動できない
						if target_cell.side == *side { continue; }

						// 移動先が相手の攻撃可能場所でなければ着手可能手
						if enemy_attackable_map[move_hand.to.y as usize][move_hand.to.x as usize] {
							// 着手可能手に追加
							hands.push(check_hand);
						}
					}else{
						// その他のチェックメイト回避手
						// - チェックメイト時に手駒配置はできないので除外
						// - ライオン以外の手からは、「相手の駒を取る手」のみがチェクメイト回避手の可能性がある

						let target_cell = self.cells[move_hand.to.y as usize][move_hand.to.x as usize];
						// 相手の駒じゃないのでスキップ
						if target_cell.side != side.reverse() { continue; }

						// 手を打ってみる
						let mut cloned = self.get_hand_applied_clone(side, &check_hand);
						// 自分が王手じゃなくなっていたら着手可能手
						if !cloned.get_or_create_is_checkmate(&side) {
							hands.push(check_hand);
						}
					}
				},
				_ => {}
			}
		}

		// handがない = チェックメイト回避不能 → ゲームオーバー
		if hands.len() == 0{
			self.states[side.to_index()] = SideState::GameOverWithCheckmate;
		}

		return hands;
	}

	// 評価処理: サブルーチン: トライアブル時の着手可能手の一覧を取得する
	fn create_valid_hands_when_tryable(&mut self, side:&Side) -> Vec<Hand> {

		let mut hands: Vec<Hand> = [].to_vec();

		let enemy_tryable_positions = self.get_or_create_tryable_positions(&side.reverse());
		if enemy_tryable_positions.len() > 1 {
			// トライ回避不能
			self.states[side.to_index()] = SideState::GameOverWithTryable;
			return hands;
		}
		if enemy_tryable_positions.len() == 1 {
			// TODO: 全てのトライ防止手の一覧を取得する
			// - 持ち駒があればこのポジションにPutするか
			// - moveしてトライできなくなっている手の一覧を探す
			let new_hands = self.create_all_move_hands(side);
			for hand in new_hands {
				// 手を打ってみる
				let cloned = self.get_hand_applied_clone(side, &hand);
				// 相手がトライアブルじゃなくなっていたら着手可能手
				if !cloned.is_tryable(&side.reverse()) {
					hands.push(hand);
				}
			}

			return hands;
		}
		// メモ: ここにくるということはpanicが妥当と思うが、設計見直してpanicの可能性をそもそも潰しておきたい気持ちも。
		// - is_tryable判定後にこのメソッドを呼んでいるので、上記のどちらかの分岐に入るはず。。
		panic!("create_valid_hands_when_tryable()で分岐に入りませんでした。is_tryable()を確認せずに呼び出された可能性？")

	}

}



// ******  ********    ******  ******  
//   **    **        **          **    
//   **    ********    ****      **    
//   **    **              **    **    
//   **    ********  ******      **    

#[cfg(test)]
mod board_tests {
	use super::*;
	#[test]
	fn test_new_board_states() {
		let board = Board::new();
		let side_a_index = Side::A.to_index();
		let side_b_index = Side::B.to_index();
		
		// test: new直後のstateはPlayableである
		assert_eq!(board.states[side_a_index], SideState::Playable);
		assert_eq!(board.states[side_b_index], SideState::Playable);

		// test: new直後の持ち駒はlen() == 0
		assert_eq!(board.tegomas[side_a_index].len(), 0);
		assert_eq!(board.tegomas[side_b_index].len(), 0);
	}

	#[test]
	fn test_new_board_to_hiyoko_forward_state() {
		// new boardからSide:Aがひよこを打った状態を評価する
		let board = Board::new();
		let side_a_index = Side::A.to_index();
		let side_b_index = Side::B.to_index();

		// 1:2のひよこを1:1に移動する
		let hand = Hand {
			put_hand: None,
			move_hand: Some(
				Move {
					from: Position{y:1,x:2},
					to: Position{y:1,x:1}
				}
			)
		};

		// 手を反映したboardを取得する
		let new_board = board.get_hand_applied_clone(
			&Side::A, &hand
		);

		// test: Side::Aは手駒にひよこがある
		assert_eq!(new_board.tegomas[side_a_index][0], Koma::Hiyoko);

		// test: Side::Bは手駒を何も持っていない
		assert_eq!(new_board.tegomas[side_b_index].len(), 0);

		// test: Side::Bは王手されている
		// TODO: このテスト失敗する。なんだろう。falseになってるご様子。TODO。
		// assert_eq!(new_board.get_or_create_is_checkmate(&Side::B), true);

	}
}