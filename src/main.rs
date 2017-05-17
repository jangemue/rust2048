mod game;
use game::game_2048::Game;
use input::user_commander::User;
use input::random_ai::RandomAI;

mod input {
	pub mod commander;
	pub mod user_commander;
	pub mod random_ai;
}

pub fn main() {
	let mut game = Game::new(4);

	let input = User::new();
	//let input = RandomAI::new();

	game.render();
	game.start(&input);
}
