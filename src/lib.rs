mod data;
mod view;

use crate::{
    data::{
        enums::{
            SideState,
            Koma,
            Side
        },
        types::{
            // Hand,
        	Cell
        },
        board::{
            Board,
        },
    },
};

#[no_mangle]
pub fn add(
	a: i32,
	b: i32
) -> i32 {
    a + b + 3
}

#[no_mangle]
pub fn get_next_hand(
	// negamax探索深さ
	max_depth: i8,
	// side。0=Side::A、1=Side::B
	side_index: i8,
	// cellの配列
	// - -1..6で、Side::Bのhiyoko-lionを表現
	// - 0は空セル。Side::Free、Koma::Null。
	// - +1..6で、Side::Aの駒を表現。
	c_a1: i8, c_b1: i8, c_c1: i8,
	c_a2: i8, c_b2: i8, c_c2: i8,
	c_a3: i8, c_b3: i8, c_c3: i8,
	c_a4: i8, c_b4: i8, c_c4: i8,
	// 手駒の配列（両陣営で合計6駒）
	// - -1..6で、Side::Bのhiyoko-lionを表現
	// - +1..6で、Side::Aの駒を表現。
	// - 空き領域は0
	tegoma1: i8,
	tegoma2: i8,
	tegoma3: i8,
	tegoma4: i8,
	tegoma5: i8,
	tegoma6: i8
) -> f64 {

	let side = match side_index {
		0 => Side::A,
		1 => Side::B,
		_ => panic!("undefined side index.")
	};

	let cells = parse_cells([
		c_a1, c_b1, c_c1,
		c_a2, c_b2, c_c2,
		c_a3, c_b3, c_c3,
		c_a4, c_b4, c_c4,
	]);

	let mut tegoma_side_a: Vec<Koma> = [].to_vec();
	let mut tegoma_side_b: Vec<Koma> = [].to_vec();

	for tegoma_i8 in [tegoma1, tegoma2, tegoma3, tegoma4, tegoma5, tegoma6]{
		let tegoma = parse_cell_i8(tegoma_i8);
		match tegoma.side {
			Side::A => {tegoma_side_a.push(tegoma.koma);}
			Side::B => {tegoma_side_b.push(tegoma.koma);}
			_ => {}
		}
	}

	let mut _board = Board{
		cells: cells,
		tegomas: [
			tegoma_side_a,
			tegoma_side_b
		],
		states: [
			SideState::Playable,
			SideState::Playable
		],
		attackable_maps: Default::default(),
		is_checkmates: Default::default(),
		tryable_positions: Default::default(),
		valid_hands: Default::default(),
	};

	let hand = _board.get_next_hand_with_negamax(max_depth,&side);

	// TODO: 4bitごとに詰めて結果を返す
	
	// - Position * 2の形で、何をどこに移動させるかを表現。
	// - 最初のPositionのxが「5」だったら持ち駒情報に分岐。
	//   - yに配置する持ち駒のインデックスが入る。
	//   - 二つ目のPositionは配置場所。
	// - [没] 最初のPositionのxが「6」だったら、yにはGameOver Resultが入っている？
	//   - TypeScript側でもゲームオーバー評価しているのでそちらに任せる。ゲームオーバー状態で呼ばれたらpanicでok
	
	// note: i8だとビットシフト演算でオーバーフローエラーになる
	let mut ret1: i32 = 0;
	let mut ret2: i32 = 0;
	let mut ret3: i32 = 0;
	let mut ret4: i32 = 0;
	let mut exist_move_hand = false;
	match hand.move_hand {
		Some(move_hand) => {
			let from = &move_hand.from;
			let to = &move_hand.to;
			ret1 = from.x as i32;
			ret2 = from.y as i32;
			ret3 = to.x as i32;
			ret4 = to.y as i32;
			exist_move_hand = true;
		}
		_ => {}
	};
	match hand.put_hand{
		Some(put_hand) => {
			ret1 = 4; // x座標は0..3なので一応一つ開けて……
			ret2 = put_hand.index as i32;
			ret3 = put_hand.to.x as i32;
			ret4 = put_hand.to.y as i32;
		}
		_ => {
			if ! exist_move_hand {
				panic!("invalid hand.")
			}
		}
	};
	return (ret1 + (ret2 << 4) + (ret3 << 8) + (ret4 << 12)) as f64;
}

// 引数のセル数値をパース
fn parse_cell_i8(num:i8) -> Cell{
	match num {
		0  => Cell{side: Side::Free, koma: Koma::Null},
		1  => Cell{side: Side::A   , koma: Koma::Hiyoko},
		2  => Cell{side: Side::A   , koma: Koma::Kirin},
		3  => Cell{side: Side::A   , koma: Koma::Zou},
		4  => Cell{side: Side::A   , koma: Koma::Niwatori},
		5  => Cell{side: Side::A   , koma: Koma::Lion},
		-1 => Cell{side: Side::B   , koma: Koma::Hiyoko},
		-2 => Cell{side: Side::B   , koma: Koma::Kirin},
		-3 => Cell{side: Side::B   , koma: Koma::Zou},
		-4 => Cell{side: Side::B   , koma: Koma::Niwatori},
		-5 => Cell{side: Side::B   , koma: Koma::Lion},
		_ => {
			panic!("ERROR: Parce Cell")
		}
	}
}

// i8の配列からcell作成
fn parse_cells(list:[i8; 12]) -> [[Cell; 3]; 4] {
	return [
		[
			parse_cell_i8(list[0]),
			parse_cell_i8(list[1]),
			parse_cell_i8(list[2])
		],
		[
			parse_cell_i8(list[3]),
			parse_cell_i8(list[4]),
			parse_cell_i8(list[5])
		],
		[
			parse_cell_i8(list[6]),
			parse_cell_i8(list[7]),
			parse_cell_i8(list[8])
		],
		[
			parse_cell_i8(list[9]),
			parse_cell_i8(list[10]),
			parse_cell_i8(list[11])
		]
	];
}
