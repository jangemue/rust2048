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

	game.render();

	let which_input = 0;
	let user = User::new();
	let random_ai = RandomAI::new();
	match which_input {
		0 => game.start(&random_ai),
		_ => game.start(&user)
	}
}
