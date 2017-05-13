pub mod game_2048 {
    pub enum Command {
        Up,Down,Left,Right
    }

    pub struct Game {
        length : usize,
        score : u32,
        field : Vec<Vec<u32>>
    }

    impl Game {
        pub fn new(length : usize) -> Game {
            let field = vec![vec![0;length]];

            Game{ length: length, score: 0, field : field}
        }

        pub fn field(&self) -> &Vec<Vec<u32>> {
            &self.field
        }

        pub fn score(&self) -> u32 {
            self.score
        }

        pub fn command(&self, command : Command) {

        }
    }
}

