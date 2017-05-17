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
            game.insert_new(Command::Up);
            game.insert_new(Command::Up);

            game
        }

        pub fn start(&mut self, commander : &Commander) {
            loop {
                let next_command = commander.next_command(self.field.to_vec());
                self.execute_command(next_command);
                let game_over = !self.insert_new(next_command) && !self.can_merge();
                self.render();
                if game_over {
                    break;
                }
            }
        }

        pub fn render(&self) {
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

        pub fn insert_new(&mut self, command : Command) -> bool {
            let possible_insert = vec![2,2,2,2,2,4,4];
            let possible_insert_between = Range::new(0, possible_insert.len());
            let mut possible_insert_rng = rand::thread_rng();
            let insert = possible_insert[possible_insert_between.ind_sample(&mut possible_insert_rng)];

            let free : Vec<(usize, usize)>;
            if command == Command::Up {
                free = self.free_fields(false, true);
            } else if command == Command::Down {
                free = self.free_fields(true, true);
            } else if command == Command::Left {
                free = self.free_fields(false, false);
            } else {
                free = self.free_fields(true, false);
            }

            if free.is_empty() {
                return false;
            }

            let row_between = Range::new(0, free.len());
            let mut row_rng = rand::thread_rng();
            let (row, column) = free[row_between.ind_sample(&mut row_rng)];

            self.field[row][column] = insert;
            return true;
        }

        fn free_fields(&self, top_down : bool, row_first : bool) -> Vec<(usize, usize)> {
            let mut free = Vec::new();
            if row_first && top_down {
                for row in 0..self.length {
                    for column in 0..self.length {
                        if self.field[row][column] == 0 {
                            free.push((row, column));
                        }
                    }
                }
            } else if row_first && !top_down {
                for row in (0..self.length).rev() {
                    for column in (0..self.length).rev() {
                        if self.field[row][column] == 0 {
                            free.push((row, column));
                        }
                    }
                }
            } else if !row_first && top_down {
                for column in 0..self.length {
                    for row in 0..self.length {
                        if self.field[row][column] == 0 {
                            free.push((row, column));
                        }
                    }
                }
            } else {
                for column in (0..self.length).rev() {
                    for row in 0..self.length  {
                        if self.field[row][column] == 0 {
                            free.push((row, column));
                        }
                    }
                }
            }

            free
        }

        fn can_merge(&self) -> bool {
            for x in 0..self.length - 1 {
                for y in 0..self.length - 1 {
                    if  self.field[x][y] == self.field[x+1][y]
                        || self.field[x][y] == self.field[x][y+1] {
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

