use super::{types::Position, constants::{KOMA_MOVERULE_HIYOKO, KOMA_MOVERULE_ZOU, KOMA_MOVERULE_KIRIN, KOMA_MOVERULE_LION, KOMA_MOVERULE_NIWATORI}};

/**
 * enmus.rs
 * - enum定義置き場
 */

// #[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Side{
	Free,
	A,
	B
}

#[allow(dead_code)]
impl Side {
	// Sideを反転させたコピーを返す
	pub fn reverse(&self) -> Side{
		match self {
			Side::A => Side::B,
			Side::B => Side::A,
			_ => panic!("Side::Freeは反転できません。")
		}
	}
	// 配列インデックスを返す
	pub fn to_index(&self) -> usize{
		match self {
			Side::A => 0,
			Side::B => 1,
			_ => panic!("Side::Freeは配列インデックスにできません。")
		}
	}
}

// #[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Koma{
	Null,
	Lion,
	Kirin,
	Zou,
	Hiyoko,
	Niwatori
}

impl Koma {
	// Side::A(下)からみた時の、コマの移動可能な座標の一覧を返す
	// - TODO: constantを毎回to_vec()していて非効率な気がするので、HashMapあたりでキャッシュしておきたい
	pub fn get_move_rule_from_side_a(&self) -> Vec<Position> {
		match self {
			Koma::Hiyoko => KOMA_MOVERULE_HIYOKO.to_vec(),
			Koma::Kirin => KOMA_MOVERULE_KIRIN.to_vec(),
			Koma::Zou => KOMA_MOVERULE_ZOU.to_vec(),
			Koma::Lion => KOMA_MOVERULE_LION.to_vec(),
			Koma::Niwatori => KOMA_MOVERULE_NIWATORI.to_vec(),
			_ => [].to_vec()
		}
	}

	// 評価用: 盤上にあった時の点数
	// memo: 過去実装の定義値をそのまま流用……エビデンスは不明
	// - 多分「持ってたらこれくらい有利かな」と言う気持ちで入れた数字
	pub fn to_onboard_score(&self) -> i32 {
		match self {
			Koma::Null => -999999, // 評価してはいけないのでエラー検知用のマイナス値にしておく
			Koma::Hiyoko => 75,
			Koma::Kirin => 100,
			Koma::Zou => 100,
			Koma::Lion => 0, // なくなる可能性はないし点評価デバッグで目障りなので0にする
			Koma::Niwatori => 110
		}
	}

	// 手駒として所持している駒の評価用点数
	// memo: 過去実装の定義値をそのまま流用……エビデンスは不明
	// - 多分「持ってたらこれくらい有利かな」と言う気持ちで入れた数字
	pub fn to_tegoma_score(&self) -> i32 {
		match self {
			Koma::Null => -999999, // 評価してはいけないのでエラー検知用のマイナス値にしておく
			Koma::Hiyoko => 100,
			Koma::Kirin => 150,
			Koma::Zou => 150,
			Koma::Lion => -999999, // 手駒になってはいけないのでエラー検知用のマイナス値にしておく
			Koma::Niwatori => -999999 // 手駒になってはいけないのでエラー検知用のマイナス値にしておく
		}
	}

}


#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum SideState {
	Playable, // ゲーム続行可能
	GameOverWithCheckmate, // 評価ターン側にチェックメイト回避手がない
	GameOverWithTryable, // 評価ターンの相手側のトライ回避手がない
	GameOverWithStalemate, // ステイルメイト=生き残れる合法手が一つもない （※wikipediaによるとこうなる可能性はないはずなのだけど、このゲームではトライアブル評価をしているので発生しうる。合法手が全てトライアブル失敗というパターン）
}



// ******  ********    ******  ******  
//   **    **        **          **    
//   **    ********    ****      **    
//   **    **              **    **    
//   **    ********  ******      **    

#[cfg(test)]
mod enums_tests {
	use super::*;

	#[test]
	fn test_side_index() {
		// test: sideのto_index()が0と1であることを確認
		assert_eq!(Side::A.to_index(), 0);
		assert_eq!(Side::B.to_index(), 1);
	}

	#[test]
	#[should_panic]
	fn test_side_free_to_index_should_panic() {
		// test: Side::Freeのto_index()はpanicする
		Side::Free.to_index();
	}

	#[test]
	fn test_side_reverse() {
		// test: Sideをreverse()した時の挙動テスト
		assert_eq!(Side::A.reverse(), Side::B);
		assert_eq!(Side::B.reverse(), Side::A);
		assert_ne!(Side::A.reverse(), Side::A);
	}

	#[test]
	#[should_panic]
	fn test_side_free_reverse_should_panic() {
		// test: Side::Freeのreverse()はpanicする
		Side::Free.reverse();
	}
}