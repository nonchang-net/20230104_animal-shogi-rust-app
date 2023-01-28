#[cfg(test)]
pub mod board_test {
	use rand::{Rng};
	use crate::data::{
		board::Board,
		enums::{Side, SideState, Koma},
		types::{
			Cell,Hand, Position, Move
		}
	};

	// ランダムに一手打って新しい盤面状態を返す
	fn play_random_hand(
		board:&mut Board,
		side:&Side
	) -> Board{
		let mut rng = rand::prelude::thread_rng();
		let hands = board.get_or_create_valid_hands(&side);
		// debug:
		// dbg!("[DEBUG] selected hand:", hands[index]);
		let index = rng.gen_range(0, hands.len());
		return board.get_hand_applied_clone(side, &hands[index]);
	}

	// 評価関数で一番高い手を打つ
	fn play_evaluate_hand(
		board:&mut Board,
		side:&Side
	) -> Board{
		let hands = board.get_or_create_valid_hands(&&side);
		let mut selected_hand = hands[0];
		let mut highscore:i32 = -99999;
		for hand in hands {
			let mut new_board = board.get_hand_applied_clone(&&side, &hand);
			let score = new_board.calculate_score(&&side.reverse()) * -1;
			if score > highscore {
				highscore = score;
				selected_hand = hand;
			}
		}
		return board.get_hand_applied_clone(&side, &selected_hand);
	}
	
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
					from: Position{y:2,x:1},
					to: Position{y:1,x:1}
				}
			)
		};

		// 手を反映したboardを取得する
		let mut new_board = board.get_hand_applied_clone(
			&Side::A, &hand
		);

		// test: Side::Aは手駒にひよこがある
		assert_eq!(new_board.tegomas[side_a_index][0], Koma::Hiyoko);

		// test: Side::Bは手駒を何も持っていない
		assert_eq!(new_board.tegomas[side_b_index].len(), 0);

		// // DEBUG: ちょっとattackable_mapを出力
		// dbg!(new_board.get_or_create_attackable_map(&Side::A));
		// dbg!(new_board.get_or_create_attackable_map(&Side::B));

		// // DEBUG:
		// println!("{}",board.render());
		// println!("{}",board.render_infomation(&Side::A));
		// println!("{}",new_board.render());
		// println!("{}",new_board.render_infomation(&Side::B));

		// test: Side::Bは王手されている
		assert_eq!(new_board.get_or_create_is_checkmate(&Side::B), true);

		// test: Side::Bのhandsは4つある。
		// - ライオンでひよこを取るか、ライオンを斜めに逃す (3手)
		// - + 象でひよこを取る (1手)。合計4手
		assert_eq!(new_board.get_or_create_valid_hands(&Side::B).len(), 4);

	}


	#[test]
	fn test_tryable_board_state_1() {
		// トライ周りがバグっているので、原因調査のため初手でトライ可能状態なマップを作成して評価していく

		let mut try_board = Board{
			cells: [
				[
					Cell{side: Side::B, koma: Koma::Lion},
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
				],
				[
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::A, koma: Koma::Lion}
				],
				[
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
				],
				[
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
				]
			],
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

		println!("{}",try_board.render());

		// test: この盤面状態で、Side::Bはトライ回避はできない
		assert_eq!(try_board.is_tryable(&Side::A), true);
		assert_eq!(try_board.get_or_create_valid_hands(&Side::B).len(), 0);
		assert_eq!(try_board.states[Side::B.to_index()], SideState::GameOverWithTryable);

	}


	#[test]
	fn test_tryable_board_state_2() {
		// Side::Bの手駒にキリンがあるのでトライを防止できるパターンの検証
		let mut try_board = Board{
			cells: [
				[
					Cell{side: Side::B, koma: Koma::Lion},
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
				],
				[
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::A, koma: Koma::Lion}
				],
				[
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
				],
				[
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
				]
			],
			tegomas:[
				[].to_vec(),
				[Koma::Kirin].to_vec(),
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

		println!("{}",try_board.render());

		// test: 手駒にキリンがあるのでトライを防止できる
		assert_eq!(try_board.is_tryable(&Side::A), true);
		assert_eq!(try_board.get_or_create_valid_hands(&Side::B).len(), 1);
		assert_eq!(try_board.states[Side::B.to_index()], SideState::Playable);

	}


	#[test]
	fn test_tryable_board_state_3() {
		// トライアブル判定ミス調査
		// - 以下の状態からSide::Aのb4のキリンがb3に移動して、Side::Bはa4にLionを移動させてしまった。
		// - この状態でSide::Aは、「キリンを動かすと負ける」と判断して、このキリンを着手可能手から省かないといけない。
		/*
			x: ａ　ｂ　ｃ　: Side.B captured
			==:============ : 
			1:🐘Ａ🦒Ａ　　 : Side.A captured
			2:　　🐥Ａ🐔Ａ : 
			3:🦁Ｂ　　　　 :
			4:🐘Ａ🦒Ａ🦁Ａ :

			Side.A's turn. hands:6 ← これがおかしい。
		*/
		let mut try_board = Board{
			cells: [
				[
					Cell{side: Side::A   , koma: Koma::Zou},
					Cell{side: Side::A   , koma: Koma::Kirin},
					Cell{side: Side::Free, koma: Koma::Null},
				],
				[
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::A   , koma: Koma::Hiyoko},
					Cell{side: Side::A   , koma: Koma::Niwatori}
				],
				[
					Cell{side: Side::B   , koma: Koma::Lion},
					Cell{side: Side::Free, koma: Koma::Null},
					Cell{side: Side::Free, koma: Koma::Null},
				],
				[
					Cell{side: Side::A   , koma: Koma::Zou},
					Cell{side: Side::A   , koma: Koma::Kirin},
					Cell{side: Side::A   , koma: Koma::Lion},
				]
			],
			tegomas:[
				[].to_vec(),
				[].to_vec(),
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

		println!("{}",try_board.render());

		// この際のhandsは6じゃない。b4 kirin→b3はトライ成功されるのでinvalid。
		// 	 - a4 zou →b3
		// 	 - c4 lion → c3
		// 	 - b1 kirin → c1
		// 	 - c2 niwatori → c1、c3
		// - → validは上記の合計の「5」が正解。
		// - → 修正done
		assert_eq!(try_board.get_or_create_valid_hands(&Side::A).len(), 5);

	}

    #[test]
    #[ignore]
    fn test_run_game_for_100_times_with_random_ai_and_random_ai() {
		// 100回回してpanicが発生しないことをテストする
		// - 普段のcargo testからは除外したいのでignore指定
		let mut current_game = 1;
		let mut current_turn = 1;
		let mut current_side = Side::A;
		let mut board = Board::new();
		loop{

			if current_game > 100 {
				println!("current game over 100. finished.");
				break;
			}
			loop{
				if current_turn > 10000 {
					panic!("current turn over 10000!");
				}
				current_turn += 1;
				board.get_or_create_valid_hands(&current_side);
		
				let side_idx = if current_side == Side::A { 0 } else { 1 };
				match board.states[side_idx] {
					SideState::Playable =>{
						// ランダムに打つ
						board = play_random_hand(&mut board, &current_side);
				
						// 次のターンに変更する
						current_turn += 1;
						current_side = current_side.reverse();
					},
					_ => {
						println!("game finish. current_game: {}", current_game);
						current_turn = 0;
						current_game += 1;
						break;
					}
				}
			}
		}

    }


    #[test]
    #[ignore]
    fn test_run_game_for_100_times_with_random_ai_and_evaluate_ai() {
		// ランダム版AIは、何度挑戦しても評価関数版AIに勝てないことを確認する
		// - 評価関数版は一手先を読むので、ランダムに打ってる相手に負ける手は打たないだろう、という先入観を検証する
		// - → 2000回ほど回したところ、一度なぜか評価関数版が負けた模様？ バグが残ってるかもしれないが再現せず。。
		let mut current_game = 1;
		let mut current_turn = 1;
		let mut current_side = Side::A;
		let mut board = Board::new();
		loop{

			if current_game > 100 {
				println!("current game over 100. finished.");
				break;
			}
			loop{
				if current_turn > 10000 {
					panic!("current turn over 10000!");
				}
				current_turn += 1;
				board.get_or_create_valid_hands(&current_side);
		
				let side_idx = if current_side == Side::A { 0 } else { 1 };
				match board.states[side_idx] {
					SideState::Playable =>{

						if current_side == Side::A {
							// サイドAはランダムに打つ
							board = play_random_hand(&mut board, &current_side);
						} else {
							// サイドBは評価関数で打つ
							board = play_evaluate_hand(&mut board, &current_side);
						}
				
						// 次のターンに変更する
						current_turn += 1;
						current_side = current_side.reverse();
					},
					_ => {
						if current_side == Side::B {
							panic!("評価関数版AIがランダムAIに敗北しました。");
						}
						println!("game finish. side {} win. current_game: {}", current_side.render(), current_game);
						current_turn = 0;
						current_game += 1;
						break;
					}
				}
			}
		}
	}


    #[test]
    #[ignore]
    fn test_run_game_for_100_times_with_random_ai_and_negamax_ai() {
		// ランダム版AIは、何度挑戦してもnegamax版AIに勝てないことを確認する
		let mut current_game = 1;
		let mut current_turn = 1;
		let mut current_side = Side::A;
		let mut board = Board::new();
		loop{

			if current_game > 100 {
				println!("current game over 100. finished.");
				break;
			}
			loop{
				if current_turn > 10000 {
					panic!("current turn over 10000!");
				}
				current_turn += 1;
				board.get_or_create_valid_hands(&current_side);
		
				let side_idx = if current_side == Side::A { 0 } else { 1 };
				match board.states[side_idx] {
					SideState::Playable =>{

						if current_side == Side::A {
							// サイドAはランダムに打つ
							board = play_random_hand(&mut board, &current_side);
						} else {
							// サイドBはnegamaxで打つ
							let hand = board.get_next_hand_with_negamax(&current_side);
							board = board.get_hand_applied_clone(&current_side, &hand);
						}
				
						// 次のターンに変更する
						current_turn += 1;
						current_side = current_side.reverse();
					},
					_ => {
						if current_side == Side::B {
							panic!("negamax版AIがランダムAIに敗北しました。");
						}
						println!("game finish. side {} win. current_game: {}", current_side.render(), current_game);
						current_turn = 0;
						current_game += 1;
						break;
					}
				}
			}
		}
	}

	// 注意: これらのAIはランダム性がないので100回評価する意味がない
    // #[test]
    // #[ignore]
    // fn test_run_game_for_100_times_with_negamax_ai_and_evaluate_ai() {
	// 	// 評価関数版AIは、何度挑戦してもnegamax版AIに勝てないことを確認する
	// 	let mut current_game = 1;
	// 	let mut current_turn = 1;
	// 	let mut current_side = Side::A;
	// 	let mut board = Board::new();
	// 	loop{

	// 		if current_game > 100 {
	// 			println!("current game over 100. finished.");
	// 			break;
	// 		}
	// 		loop{
	// 			if current_turn > 10000 {
	// 				panic!("current turn over 10000!");
	// 			}
	// 			current_turn += 1;
	// 			board.get_or_create_valid_hands(&current_side);
		
	// 			let side_idx = if current_side == Side::A { 0 } else { 1 };
	// 			match board.states[side_idx] {
	// 				SideState::Playable =>{

	// 					if current_side == Side::A {
	// 						// サイドAはnegamaxで打つ
	// 						let hand = board.get_next_hand_with_negamax(&current_side);
	// 						board = board.get_hand_applied_clone(&current_side, &hand);
	// 					} else {
	// 						// サイドBは評価関数で打つ
	// 						board = play_evaluate_hand(&mut board, &current_side);
	// 					}
				
	// 					// 次のターンに変更する
	// 					current_turn += 1;
	// 					current_side = current_side.reverse();
	// 				},
	// 				_ => {
	// 					if current_side == Side::A {
	// 						panic!("negamax AIが評価関数版AIに敗北しました。");
	// 					}
	// 					println!("game finish. side {} win. current_game: {}", current_side.render(), current_game);
	// 					current_turn = 0;
	// 					current_game += 1;
	// 					break;
	// 				}
	// 			}
	// 		}
	// 	}
	// }

}