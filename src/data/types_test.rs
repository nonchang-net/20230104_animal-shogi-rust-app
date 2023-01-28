#[cfg(test)]
mod types_test {
	use crate::data::{constants::*, types::Position, enums::Side};

	#[test]
	fn test_position_side_add() {
		// test: position.add()のsideごとの挙動を確認する
		let position_a = Position {y: 1, x: 1};
		// ひよこの移動手を試す
		let move_position = KOMA_MOVERULE_HIYOKO[0];
		// test: Side::Aなら上に行く
		assert_eq!(position_a.add(&move_position, &Side::A), Position {y:0, x:1});
		// test: Side::Bなら下に行く
		assert_eq!(position_a.add(&move_position, &Side::B), Position {y:2, x:1});
	}

	#[test]
	fn test_position_is_valid() {
		// 盤上判定が正しいかチェック
		assert_eq!(Position {y:0, x:0}.is_valid(), true);
		assert_eq!(Position {y:3, x:2}.is_valid(), true);

		assert_eq!(Position {y: 0,x:-1}.is_valid(), false);
		assert_eq!(Position {y:-1,x: 0}.is_valid(), false);
		assert_eq!(Position {y: 0,x: 3}.is_valid(), false);
		assert_eq!(Position {y: 4,x: 0}.is_valid(), false);
	}

}
