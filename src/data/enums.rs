// #[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum Side{
	Free,
	A,
	B
}

#[allow(dead_code)]
impl Side {
	pub fn reverse(&self) -> Side{
		// note: なんでこう書けないんだろ？
		// if self == Side::A {
		//     return Side::B
		// }else{
		//     return Side::A
		// }
		match self {
			Side::A => Side::B,
			Side::B => Side::A,
			_ => Side::Free
		}
	}
}

// #[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum Koma{
	Null,
	Lion,
	Kirin,
	Zou,
	Hiyoko,
	Niwatori
}