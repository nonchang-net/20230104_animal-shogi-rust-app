/**
 * Board.rs
 * - 盤面状態と手駒状態のセット
 * - 各種評価メソッドと、評価済み情報のSideごとのOptionの配列を保持する
 */

// extern crate board;

use crate::data::{types::{
	Cell,
}, constants::ENABLE_MOVE_SCORE};

use super::{enums::{Koma, Side, SideState}, constants::{INITIAL_BOARD_DATA, ATTACKABLE_POS_SCORE, TRYABLE_SCORE, CHECKMATE_SCORE, LION_LINE_SCORE}, types::{Position, Hand, Put, Move}};



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

	// flagなセルの個数を数える
	fn count_flags(&self, flag:bool) -> i32 {
		let mut count = 0;
		for x in 0..3 {
			for y in 0..4 {
				if self.data[y][x] == flag { count += 1; }
			}
		}
		return count;
	}
}


// Boardの全てのCellを順に返すイテレータ
// impl Iterator for Board{
// 	type Item = Cell;
// 	fn next(&mut self) -> Option<Self::Item> {
// 		self.iter_x += 1;
// 		if self.iter_x >= 3 {
// 			self.iter_x = 0;
// 			self.iter_y += 1;
// 			if self.iter_y >=4 {
// 				// 終了している
// 				return None;
// 			}
// 		}
// 		return Some(self.cells[self.iter_y][self.iter_x]);
// 	}
// 	fn size_hint(&self) -> (usize, Option<usize>) {
// 		return (12, Some(12));
// 	}
// }

// impl ExactSizeIterator for Board{
// 	fn len(&self) -> usize { return 12; }
// }

#[allow(dead_code, unused_variables)]
#[derive(Debug, Clone)]
pub struct Board{
	// 盤情報の二次元配列
	pub cells: [[Cell; 3]; 4],

	// 手駒: sideをキーにしたKomaの配列
	pub tegomas: [Vec<Koma>; 2],

	// イテレータの現在処理位置
	// TODO: イテレータ実装をトレイトあたりで分けてBoard自体から省きたいかな。評価時にcloneする意味がない
	// iter_x: usize,
	// iter_y: usize,

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
			// iter_x: 0,
			// iter_y: 0,
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
				if from_cell.side != *side {
					panic!("想定外呼び出し: 自分の駒以外をmoveしようとしました。。");
				}

				// 相手のコマを取ったかどうか
				if to_cell.side != Side::Free {
					// 手駒に追加
					// 鶏を取った際はヒヨコに戻す
					let new_koma = if to_cell.koma == Koma::Niwatori { Koma::Hiyoko } else { to_cell.koma };
					match to_cell.side {
						Side::A => {
							tegoma_side_b.push(new_koma);
						},
						Side::B => {
							tegoma_side_a.push(new_koma);
						},
						_ => {
							panic!("ここには来ないはず")
						}
					}
					
				}

				// 奥まで進んだヒヨコはニワトリにする
				// undone: どうぶつしょうぎにはヒヨコ打ちが有効な局面が存在するという話もあるので、この処理は若干ロスかもしれない。実装の簡便さを優先
				let try_y:i8 = if *side == Side::A { 0 } else { 3 };
				if to.y == try_y && from_cell.koma == Koma::Hiyoko {
					new_cells[to.y as usize][to.x as usize] = Cell{
						side:from_cell.side,
						koma:Koma::Niwatori
					};
				}else{
					// 移動先を移動元の駒に置き換える
					new_cells[to.y as usize][to.x as usize] = Cell{
						side:from_cell.side,
						koma:from_cell.koma
					};
				}
				
				// 移動元を空白に置き換える
				new_cells[from.y as usize][from.x as usize] = Cell{
					side:Side::Free,
					koma:Koma::Null
				};

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
			// iter_x: 0,
			// iter_y: 0,
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

	// ライオンの進捗評価
	// - 一段上がることに評価関数の点数を上げるための算出
	fn get_lion_progress(&self, side:&Side) -> i32 {
		let lion_pos = self.search_lion_pos(side);
		if *side == Side::A {
			// 下から見ると、y座標を3で引いた値が進捗になる
			return 3 - lion_pos.y as i32;
		}
		return lion_pos.y as i32;
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
	pub fn is_tryable(&mut self, side:&Side) -> bool {
		let count = self.get_or_create_tryable_positions(side).len();
		return count > 0;
	}

	// 評価処理: sideがトライ可能か確認
	fn get_or_create_tryable_positions(&mut self, side:&Side) -> Vec<Position> {
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
	fn create_tryable_positions(&mut self, side:&Side) -> Vec<Position> {
		let mut results: Vec<Position> = [].to_vec();

		// トライ可能なライン
		let tryable_y:i8 = if *side == Side::A { 1 } else { 2 };
		let lion_pos = self.search_lion_pos(side);

		// ライオンがトライ可能位置にいなければreturn
		if lion_pos.y != tryable_y { return results; }

		// トライ目標のライン
		let try_y:i8 = if *side == Side::A { 0 } else { 3 };

		let enemy_attackable_map = self.get_or_create_attackable_map(&side.reverse()).data;

		for x in 0..3{
			// トライ目標座標xがライオンの動ける範囲外かチェック
			if lion_pos.x - 1 > x || lion_pos.x + 1 < x { continue; }

			let target_cell = self.cells[try_y as usize][x as usize];

			// 自分の駒がある場所には移動できない
			if target_cell.side == *side { continue; }

			// 相手が攻撃可能な場所はtryableではない
			if enemy_attackable_map[try_y as usize][x as usize] { continue; }

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
				// println!("DEBUG: get_or_create_valid_hands() get cached. side: {}", side.render());
				return x.clone()
			},
			_ => {
				// キーが存在しないので新規作成
				// println!("DEBUG: get_or_create_valid_hands() new created. side: {}", side.render());
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

		// 相手がトライ可能時
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

				let move_hand = Move{
					from: pos,
					to: target_pos
				};
				let hand = Hand{
					move_hand: Some(move_hand),
					put_hand: None
				};

				// 手を動かしてみて、相手がトライ成功できる状態になる手は省く
				let mut cloned = self.get_hand_applied_clone(side, &hand);
				if cloned.is_tryable(&side.reverse()) {
					continue;
				}

				// 着手可能手に追加
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
						if !enemy_attackable_map[move_hand.to.y as usize][move_hand.to.x as usize] {
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

		// println!("到達チェック: create_valid_hands_when_tryable()");

		let mut hands: Vec<Hand> = [].to_vec();

		let enemy_tryable_positions = self.get_or_create_tryable_positions(&side.reverse());

		// DEBUG
		// println!("[DEBUG] side={} トライ可能場所一覧: {}", side.render(), enemy_tryable_positions.len());

		if enemy_tryable_positions.len() > 1 {
			// トライ回避不能
			self.states[side.to_index()] = SideState::GameOverWithTryable;
			return hands;
		}
		if enemy_tryable_positions.len() == 1 {
			// TODO: 全てのトライ防止手の一覧を取得する
			// - 持ち駒があればこのポジションにPutするか
			// - moveしてトライできなくなっている手の一覧を探す
			let mut new_hands = self.create_all_move_hands(side);
			let new_put_hands = &mut self.create_all_put_hands(side);
			new_hands.append(new_put_hands);

			// DEBUG
			// println!("[DEBUG] new_put_hands() len: {}", new_put_hands.len());

			for hand in new_hands {
				// 手を打ってみる
				let mut cloned = self.get_hand_applied_clone(side, &hand);

				// 相手がトライアブルじゃなくなっていたら着手可能手
				if !cloned.is_tryable(&side.reverse()) {
					hands.push(hand);
				}
			}

			if hands.len() == 0 {
				// 手がないのでトライ回避不能
				self.states[side.to_index()] = SideState::GameOverWithTryable;
			}

			return hands;
		}
		// メモ: ここにくるということはpanicが妥当と思うが、設計見直してpanicの可能性をそもそも潰しておきたい気持ちも。
		// - is_tryable判定後にこのメソッドを呼んでいるので、上記のどちらかの分岐に入るはず。。
		panic!("create_valid_hands_when_tryable()で分岐に入りませんでした。is_tryable()を確認せずに呼び出された可能性？")

	}






	//   ******    ****      ****    ******    ********  
	// **        **    **  **    **  **    **  **        
	//   ****    **        **    **  ******    ********  
	//       **  **    **  **    **  **  **    **        
	// ******      ****      ****    **    **  ********  


	// 評価関数
	// - boardをsideのターンとして評価したスコアを返す
	// - 過去の実装をそのまま持ってきたのでエビデンスは不明
	// - 5桁スコアは勝利確定として扱ってた様子
	pub fn calculate_score (&mut self, side:&Side) -> i32 {

		// 盤面状態評価
		self.get_or_create_valid_hands(side);
		
		// 勝敗状態を返却
		if self.states[side.to_index()] != SideState::Playable {
			// println!("calc: {} は敗北しています。", side.render());
			return -99999;
		}
		if self.states[side.reverse().to_index()] != SideState::Playable {
			// println!("calc: {} は勝利状態です。", side.render());
			return 99999;
		}

		// 点数計算開始
		let mut score = 0;

		// TODO: イテレータ書きたいが上手くいかないのでちょっとコメントアウト中
		// for cell in self {
		// }

		// 盤上の駒の点数をside毎に評価
		for x in 0..3{
			for y in 0..4{
				let cell = self.cells[y][x];
				if cell.side == Side::Free { continue; }
				let is_own = if cell.side == *side { 1 } else { -1 };
				score += cell.koma.to_onboard_score() * is_own;
			}
		}

		// 手駒の点数を評価
		for tegoma in self.tegomas[side.to_index()].iter() {
			score += tegoma.to_tegoma_score();
		}
		for tegoma in self.tegomas[side.reverse().to_index()].iter() {
			score -= tegoma.to_tegoma_score();
		}

		// 着手可能手の多さを評価
		score += self.get_or_create_valid_hands(side).len() as i32 * ENABLE_MOVE_SCORE;
		// note: 以下を評価したらライオンを取ってpanic
		// - この評価は番手のみでokそう
		// score -= self.get_or_create_valid_hands(&side.reverse()).len() as i32 * ENABLE_MOVE_SCORE;

		// 効いてる場所の数を点数に加える
		score += self.get_or_create_attackable_map(side).count_flags(true) * ATTACKABLE_POS_SCORE;
		score -= self.get_or_create_attackable_map(&side.reverse()).count_flags(true) * ATTACKABLE_POS_SCORE;

		// Lionのトライ可能性評価で1ラインごとに加算
		score += self.get_lion_progress(side) * LION_LINE_SCORE;
		score -= self.get_lion_progress(&side.reverse()) * LION_LINE_SCORE;

		// チェックメイト時は一定点数加算
		// - この評価は番手のみ
		score += if self.get_or_create_is_checkmate(side) { CHECKMATE_SCORE } else { 0 };

		// 敵がトライ可能な時は一定点数減算
		// - この評価は番手のみ
		score -= if self.is_tryable(&side.reverse()) { TRYABLE_SCORE } else { 0 };
		
		return score;
	}


	// 盤面状態を元に、次の1手をnegamax先読みして決定する
	pub fn get_next_hand_with_negamax(
		&mut self,
		side: &Side
	) -> Hand {
		let hands = self.get_or_create_valid_hands(&side);
		let mut selected_hand = hands[0];
		let mut highscore = -99999;
		for hand in hands {
			let mut new_board = self.get_hand_applied_clone(&side, &hand);
			let score = self::get_negamax_score(
				0,
				3,
				&mut new_board,
				side.reverse(),
				50000
			);
			if score > highscore {
				highscore = score;
				selected_hand = hand;
			}
		}
		return selected_hand;
	}

}


// negamax本体
// - 再起処理で先読み累計スコアを返す
// - 間違って内部状態を持たせたくないのでstruct Board外に移動
pub(crate) fn get_negamax_score(
	depth: i8,
	max_depth: i8,
	board: &mut Board,
	current_side: Side,
	limit_score: i32
) -> i32 {
	let new_depth = depth + 1;
	let current_score = board.calculate_score(&current_side);

	// 	if(TemporaryState.evaluateCount > 1000000){
	// 		console.log(`evaluateCount > 1000000: 処理が多いので一旦停止`)
	// 		return this.Score()
	// 	}

	// 停止条件評価
	if new_depth > max_depth ||
		current_score > limit_score ||
		current_score < (limit_score * -1)
	{
		// DEBUG:
		// println!("[DEBUG] negamax return. depth:{}, score:{}", new_depth, current_score);
		return current_score;
	}

	let mut high_score = -999999;

	let hands = board.get_or_create_valid_hands(&current_side);
	for hand in hands {
		let mut new_board = board.get_hand_applied_clone(&current_side, &hand);

		// 符号反転して相手側スコアを再起評価
		let new_score = -1 * self::get_negamax_score(
			new_depth,
			max_depth,
			&mut new_board,
			current_side.reverse(),
			limit_score
		);

		if new_score > high_score {
			high_score = new_score;
		}
	}
	return high_score;
}
