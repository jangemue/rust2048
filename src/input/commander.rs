#[derive(PartialEq,Clone,Copy)]
pub enum Command {
    Up,
    Down,
    Left,
    Right
}

pub trait Commander {
    fn next_command(&self, fields : Vec<Vec<u32>>) -> Command;
}