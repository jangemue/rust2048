use input::commander::Commander;
use input::commander::Command;
use std::io;
use std::io::BufRead;

pub struct User {

}

impl User {
    pub fn new() -> User {
        User { }
    }
}

impl Commander for User {
    #[allow(unused_variables)]
    fn next_command(&self, fields : Vec<Vec<u32>>) -> Command {
        let input = io::stdin();
        for line in input.lock().lines() {
            match line.unwrap().as_ref() {
                "w" => return Command::Up,
                "s" => return Command::Down,
                "d" => return Command::Right,
                "a" => return Command::Left,
                _ => continue
            }
        }
        return Command::Up;
    }
}