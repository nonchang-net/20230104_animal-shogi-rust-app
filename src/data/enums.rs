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
			_ => Side::Free
		}
	}
}

// #[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Koma{
	Null,
	Lion,
	Kirin,
	Zou,
	Hiyoko,
	Niwatori
}