/**
 * CUI実装のエントリポイント
 */

use animal_shogi_rust_app::GameRunner;

fn main() {
	let mut _game = GameRunner::new();
	_game.start();
}