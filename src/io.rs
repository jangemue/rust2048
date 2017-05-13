

pub mod user_io {
    use game::game_2048;
    use std::io;
    use std::io::BufRead;
    use std::thread;
    use std::convert::AsRef;



    pub fn start_user_io(g: game_2048::Game) {
        let stdin = io::stdin();
        let handle = thread::spawn(move || {
            for line in stdin.lock().lines() {
                let l = line.unwrap();
                if l == "w" {
                    &g.command(game_2048::Command::Up);
                }
                if l == "s" {
                    &g.command(game_2048::Command::Down);
                }
                if l == "a" {
                    &g.command(game_2048::Command::Left);
                }
                if l == "d" {
                    &g.command(game_2048::Command::Right);
                }
                if l == "x" {
                    break;
                }
            }
            return "good by";
        });

        println!("{}", handle.join().unwrap());
    }

}

