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
			_ => Side::Free
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
}
