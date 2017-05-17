extern crate rand;
use input::commander::Commander;
use input::commander::Command;
use self::rand::distributions::{IndependentSample, Range};

pub struct RandomAI {

}

impl RandomAI {
    pub fn new() -> RandomAI {
        RandomAI{ }
    }
}

impl Commander for RandomAI {
    #[allow(unused_variables)]
    fn next_command(&self, fields : Vec<Vec<u32>>) -> Command {
        let commands = vec![Command::Up, Command::Down, Command::Left, Command::Right];
        let between = Range::new(0, commands.len());
        let mut commands_rng = rand::thread_rng();

        commands[between.ind_sample(&mut commands_rng)]
    }
}

