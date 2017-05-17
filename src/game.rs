pub mod game_2048 {
extern crate rand;
use self::rand::distributions::{IndependentSample, Range};
use input::commander::Command;
use input::commander::Commander;

    pub struct Game {
        length: usize,
        score: u32,
        field: Vec<Vec<u32>>
    }

    impl Game {
        pub fn new(length: usize) -> Game {
            let field = vec![vec![0; length]; length];

            let mut game = Game {
                length: length,
                score: 0,
                field: field
            };
            game.insert_new();
            game.insert_new();

            game
        }

        pub fn start(&mut self, commander : &Commander) {
            loop {
                let next_command = commander.next_command(self.field.to_vec());
                self.execute_command(next_command);
                let can_merge = self.can_merge();
                let insert_new = self.insert_new();
                let game_over = !can_merge && !insert_new;
                self.render();
                println!("Merge: {}  -  New: {}", can_merge, insert_new);
                if game_over {
                    break;
                }
            }
        }

        pub fn render(&self) {
            print!("{}[2J", 27 as char);
            println!("Score: {:6}", self.score);
            for r in 0..self.length {
                for _ in 0..self.length {
                    print!("+----");
                }
                print!("+");
                print!("\n");
                print!("|");
                for c in 0..self.length {
                    if self.field[r][c] == 0{
                        print!("    |");
                    } else {
                        print!("{:4}|", self.field[r][c]);
                    }
                }
                print!("\n")
            }
            for _ in 0..self.length {
                print!("+----");
            }
            print!("+");
            print!("\n");
        }

        pub fn insert_new(&mut self,) -> bool {
            let mut free = Vec::new();
            for i in 0..self.length {
                for j in 0..self.length {
                    if self.field[i][j] == 0{
                        free.push((i,j));
                    }
                }
            }

            if free.is_empty() {
                return false;
            }

            let insert_between = Range::new(0, 10);
            let mut insert_rng = rand::thread_rng();
            let insert = if insert_between.ind_sample(&mut insert_rng) > 7 {
                4
            } else {
                2
            };

            let row_between = Range::new(0, free.len());
            let mut row_rng = rand::thread_rng();
            let (row, column) = free[row_between.ind_sample(&mut row_rng)];

            self.field[row][column] = insert;

            true
        }


        fn can_merge(&self) -> bool {
            for x in 0..self.length {
                for y in 0..self.length {
                    if  (x < self.length - 1 && self.field[x][y] == self.field[x+1][y])
                        || (y < self.length - 1 && self.field[x][y] == self.field[x][y+1]) {
                        return true;
                    }
                }
            }

            false
        }

        fn execute_command(&mut self, cmd : Command) {
            loop {
                let mut moved = false;
                for i in 0..self.length {
                    for j in 0..self.length {
                        moved |= self.move_field_guarded(cmd, i, j);
                    }
                }

                if !moved {
                    break;
                }
            }
        }

        fn move_field_guarded(&mut self, cmd : Command, i : usize, j : usize) -> bool {
            if (cmd == Command::Up && i > 0)
                || (cmd == Command::Down && i < self.length - 1)
                || (cmd == Command::Left && j > 0)
                || (cmd == Command::Right && j < self.length - 1){
                return self.move_field(cmd, i, j);
            } else {
                return false
            }
        }

        fn move_field(&mut self, cmd : Command, i : usize, j : usize) -> bool {
            // Is it a valued field?
            if self.field[i][j] == 0 {
                return false
            }

            // Find neighbour.
            let (neighbour_i, neighbour_j) = self.get_neighbour_index(cmd, i, j);

            // Could join?
            let neighbour_value = self.field[neighbour_i][neighbour_j];
            if neighbour_value == self.field[i][j] {
                // Set new value to neighbour.
                self.field[neighbour_i][neighbour_j] = 2 * neighbour_value;
                // Truncate me.
                self.field[i][j] = 0;
                // Add score.
                self.score += 2 * neighbour_value;

                return true
            } else if neighbour_value == 0 {    // Is neighbour empty?
                // Move me to neighbour.
                self.field[neighbour_i][neighbour_j] = self.field[i][j];

                // Truncate me.
                self.field[i][j] = 0;

                return true
            }

            return false
        }

        fn get_neighbour_index(&self, cmd : Command, i : usize, j : usize) -> (usize, usize) {
            match cmd {
                Command::Up => (i-1, j),
                Command::Down => (i+1, j),
                Command::Left => (i, j-1),
                _ => (i, j+1)
            }
        }
    }
}

