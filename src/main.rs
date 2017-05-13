mod io;
mod game;
use io::user_io::start_user_io;
use game::game_2048::Game;

pub fn main() {
	println!("Hello 2048");
	let g = Game::new(4);
	start_user_io(g);
}
