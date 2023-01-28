/**
 * CUI実装のエントリポイント
 */

mod data;
mod view;
mod runtime_tests;
use runtime_tests::runner::GameRunner;

fn main() {
	let mut _game = GameRunner::new();
	_game.start();
}