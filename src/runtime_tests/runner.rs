use rand::prelude::*;

use crate::{
    data::{
        enums::{
            SideState,
            // Koma,
            Side
        },
        types::{
            Hand
        	// Cell,
        },
        board::{
            Board,
        },
    },
};

// ゲーム挙動実装とコンピューター判断のテスト用struct/impl
pub struct GameRunner{
    board: Board,
    current_side: Side,
    current_turn: u32,
    rng: ThreadRng,
}

#[allow(dead_code)]
impl GameRunner{
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
            // 入力待機
            // - ここをコメントアウトすると一気に決着がつく。暴走注意
            let answer = Self::get_input();
            if answer == "q" { break; }

            // 暴走防御
            if self.current_turn > 10000 { break; }

            // ゲームオーバー評価
            // - get_or_create_valid_hands()評価で負けてるかどうかを判定
            // TODO: 直感的じゃないな。。get_or_create_state()を作って、boardのstatesはprivateにしちゃおうか。
            self.board.get_or_create_valid_hands(&self.current_side);
            let side_idx = if self.current_side == Side::A { 0 } else { 1 };
            match self.board.states[side_idx] {
                SideState::Playable =>{
                    // 次ターン評価
                    self.next();
                },
                _ => {
                    self.print_gameover_message(&self.board.states[side_idx]);
                    break;
                }
            }
        }
    }

    fn print_gameover_message(&self, side_state: &SideState) {
        match side_state {
            SideState::GameOverWithCheckmate => {
                println!("GAME OVER: チェックメイト回避手がありませんでした。");
            },
            SideState::GameOverWithTryable => {
                println!("GAME OVER: 相手のトライを回避できない状態でした。");
            },
            SideState::GameOverWithStalemate => {
                println!("GAME OVER: 有効な手がありませんでした。");
            },
            _ => {
                panic!("想定外動作: playableなのにprint_gameoverに来た？")
            }
        }
    }

    // 相手ターンにして一手進める
    pub fn next(&mut self) {

        // テスト中のAIパターン色々
        // let hand = self.get_random_ai_hand();
        // let hand = self.get_highscore_ai_hand();
        // let hand = self.get_highscore_ai_with_random();
        // let hand = self.board.get_next_hand_with_negamax(&self.current_side);
        // self.board = self.board.get_hand_applied_clone(&self.current_side, &hand);

        // Side::AとBでAIを分けてみる
        if self.current_side == Side::A {
            let hand = self.board.get_next_hand_with_negamax(&self.current_side);
            self.board = self.board.get_hand_applied_clone(&self.current_side, &hand);
        }else{
            let hand = self.get_highscore_ai_with_random();
            self.board = self.board.get_hand_applied_clone(&self.current_side, &hand);
        }

        // 次のターンに変更する
        self.current_turn += 1;
        self.current_side = self.current_side.reverse();
    }

    // たまにランダムに打つAI（panic評価テスト用）
    fn get_highscore_ai_with_random(&mut self) -> Hand {
        if self.rng.gen_bool(0.5) {
            return self.get_highscore_ai_hand();
        }
        return self.get_random_ai_hand();
    }

    // 着手可能手から一番いいスコアの手を返すAI
    fn get_highscore_ai_hand(&mut self) -> Hand {

        let hands = self.board.get_or_create_valid_hands(&self.current_side);
        let mut selected_hand = hands[0];
        let mut highscore:i32 = -99999;
        for hand in hands {
            
            // DEBUG
            // dbg!(hand);

            // 手を打ってみる
            let mut new_board = self.board.get_hand_applied_clone(&self.current_side, &hand);

            // DEBUG: dump
            // println!("======評価中");
            // println!("{}",new_board.render());
            // println!("============");

            // 相手側からの評価を取得して符号反転
            let score = new_board.calculate_score(&self.current_side.reverse()) * -1;
            if score > highscore {
                highscore = score;
                selected_hand = hand;
            }
        }
        return selected_hand;
    }

    // ランダムな手を一つ選択するAI
    fn get_random_ai_hand(&mut self) -> Hand {

        let hands = self.board.get_or_create_valid_hands(&self.current_side);
        let index = self.rng.gen_range(0, hands.len());

        // debug:
        // dbg!("[DEBUG] selected hand:", hands[index]);

        return hands[index].clone();
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
