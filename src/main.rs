#![allow(unused)]

mod data;

use crate::data::enums::{
    Koma,
    Side
};
use crate::data::types::{
    Board,
    Cell,
};


fn main() {

    // Board構造体の初期化テスト
    // let _board = Board{
    //     data: [
    //         [
    //             Cell{side: Side::A, koma:Koma::Null},
    //             Cell{side: Side::A, koma:Koma::Null},
    //             Cell{side: Side::A, koma:Koma::Null}
    //         ],
    //         [
    //             Cell{side: Side::A, koma:Koma::Null},
    //             Cell{side: Side::A, koma:Koma::Null},
    //             Cell{side: Side::A, koma:Koma::Null}
    //         ],
    //         [
    //             Cell{side: Side::A, koma:Koma::Null},
    //             Cell{side: Side::A, koma:Koma::Null},
    //             Cell{side: Side::A, koma:Koma::Null}
    //         ],
    //         [
    //             Cell{side: Side::A, koma:Koma::Null},
    //             Cell{side: Side::A, koma:Koma::Null},
    //             Cell{side: Side::A, koma:Koma::Null}
    //         ]
    //     ],
    //     // TODO: HashMapの初期値はどう書けばいいのかな
    //     // tegomas: [
    //     //     {Side::A, new Vec<Koma>},
    //     //     {Side::A, new Vec<Koma>}
    //     // ]
    // };

    // Cell構造体の初期化テスト
    let _cell = Cell{
        side: Side::A,
        koma: Koma::Hiyoko,
    };

    // これはなんでできないんだろ
    // dbg!(board)

    // println!("{:?}", _board);
    println!("{:?}", _cell);

    render();
    let answer = get_input();
    println!("{}", answer);
}

fn get_input() -> String {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}

fn render() {
    println!();
    println!("animal shogi: ver20230104.2006");
    println!();
    println!("  : ａ　ｂ　ｃ　: ----------------");
    println!("==:============ : Side.B captured:");
    println!(" 1:🐘Ｂ🦁Ｂ🦒Ｂ : none");
    println!(" 2:　　🐥Ｂ　　 : ----------------");
    println!(" 3:　　🐥Ａ　　 : Side.A captured:");
    println!(" 4:🦒Ａ🦁Ａ🐘Ａ : none");
    println!();
    println!("Side.A's turn. YOU ARE CHECKMATED!!!");
    println!("command: (? to show help. q to quit)");
}