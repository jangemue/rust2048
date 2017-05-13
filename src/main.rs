mod io;
mod game;
use io::user_io::start_user_io;
use game::game_2048::Game;

pub fn main() {
	let g = Game::new(4);
	println!("Score: {}", g.score());
	g.render();
	start_user_io(g);
}
