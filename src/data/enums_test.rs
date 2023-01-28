#[cfg(test)]
mod enums_test {
	use crate::data::enums::Side;

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