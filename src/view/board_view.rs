
use crate::data::enums::{
	// Koma,
	Side
};
// use crate::data::types::{
// 	Cell,
// };
use crate::data::board::{
	Board,
};

// Boardの表示用impl
impl Board {

	// 盤面情報と持ち駒情報のレイアウト全体を出力
	pub fn render(&self) -> String {
		let mut result = String::new();

		// ヘッダーとstatus枠表示
		result.push_str("  : ａ　ｂ　ｃ　: Side.B captured\n");
		result.push_str("==:============ : ");
		result.push_str(self.render_motigoma(Side::B).as_str());
		result.push('\n');

		// セル表示開始
		for (index, line) in self.data.iter().enumerate() {
			result.push_str(format!(" {}:", index+1).as_str());
			for cell in line.iter() {
				result.push_str(cell.render().as_str())
			}
			// ステータス枠表示
			match index {
				0 => result.push_str(" : Side.A captured\n"),
				1 => {
					result.push_str(" : ");
					result.push_str(self.render_motigoma(Side::A).as_str());
					result.push('\n');
				},
				_ => result.push_str(" :\n")
			}
		}
		return result;
	}
	
	// 持ち駒列を出力
	pub fn render_motigoma(&self, side:Side) -> String {
		let mut result = String::new();
		let tegomas = self.tegomas.borrow();
		let komalist = tegomas.get(&side);
		match komalist {
			Some(x) => for koma in x {
				result.push(koma.render())
			},
			None => result.push_str("none")
		}
		return result;
	}

	// 操作説明枠を表示
	pub fn render_infomation(&self, side:Side) -> String {
		let mut result = String::new();
		let is_checkmate = false;
		let checkmate_str = String::from("is checkmate.");
		result.push_str(format!(
			"Side.{}'s turn. {}\n",
			side.render(),
			if is_checkmate {
				checkmate_str
			} else {
				String::from("")
			}
		).as_str());

		return result;
	}

	// pub fn test2() -> &'static str{
	// 	"test"
	// }

}