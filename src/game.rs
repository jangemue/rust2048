pub mod game_2048 {
extern crate rand;
use self::rand::distributions::{IndependentSample, Range};

    #[derive(PartialEq,Clone,Copy)]
    pub enum Command {
        Up,
        Down,
        Left,
        Right
    }

    pub struct Game {
        length: usize,
        score: u32,
        field: Vec<Vec<u32>>
    }

    impl Game {
        pub fn new(length: usize) -> Game {
            let mut field = vec![vec![0; length]; length];

            let mut game = Game {
                length: length,
                score: 0,
                field: field
            };
            game.insert_new(Command::Up);
            game.insert_new(Command::Up);

            game
        }

        pub fn score(&self) -> u32 {
            self.score
        }

        pub fn command(&mut self, command: Command) {
            self.on_command(command);
            self.insert_new(command);
            self.render();
        }

        pub fn command2(&mut self, command: Command) {
            let (x_mod,x_start,x_border,y_mod,y_start,y_border, x_first) = self.direction(command);

            //get start x dep on direction
            //get start y dep on direction
            if x_first {
                let mut x: usize = x_start;
                while x!=x_border {
                    let mut y: usize = y_start;
                    while y!=y_border {
                        self.merge(x,y,command);
                        if y_mod < 0 {
                            y-=1;
                        } else {
                            y+=1;
                        }
                        //y+=y_mod;
                    }
                    if x_mod < 0 {
                        x-=1;
                    } else {
                        x+=1;
                    }
                }

            } else {
                let mut y: usize = y_start;
                while y!=y_border {
                    let mut x: usize = x_start;
                    while x!=x_border {
                        self.merge(x,y,command);
                        if x_mod < 0 {
                            x-=1;
                        } else {
                            x+=1;
                        }
                    }
                    if y_mod < 0 {
                        y-=1;
                    } else {
                        y+=1;
                    }
                }
            }
            //move all dep on direction
            if x_first {
                let mut x = x_start;
                while x!=x_border {
                    let mut y = y_start;
                    while y!=y_border {
                        self.move_fields(x,y,command);
                        if y_mod < 0 {
                            y-=1;
                        } else {
                            y+=1;
                        }
                    }
                    if x_mod < 0 {
                        x-=1;
                    } else {
                        x+=1;
                    }
                }

            } else {
                let mut y = y_start;
                while y!=y_border {
                    let mut x = x_start;
                    while x!=x_border {
                        self.move_fields(x,y,command);
                        if x_mod < 0 {
                            x-=1;
                        } else {
                            x+=1;
                        }
                    }
                    if y_mod < 0 {
                        y-=1;
                    } else {
                        y+=1;
                    }
                }
            }
        }

        fn move_fields(&mut self,x: usize,y: usize,command: Command){
            let my_score = self.field[x][y];
            if my_score==0 {
                //move all others in the lane
                if command == Command::Up {
                    for i_y in y+1..self.length {
                        self.field[x][i_y-1]=self.field[x][i_y];
                        self.field[x][i_y]=0;
                    }
                }
                if command == Command::Down {
                    for i_y in (0..y-1).rev() {
                        self.field[x][i_y+1]=self.field[x][i_y];
                        self.field[x][i_y]=0;
                    }
                }
                if command == Command::Right {
                    for i_x in (0..x-1).rev() {
                        self.field[i_x][y]=self.field[i_x][y];
                        self.field[i_x][y]=0;
                    }
                }
                if command == Command::Left {
                    for i_x in x+1..self.length {
                        self.field[i_x][y]=self.field[i_x][y];
                        self.field[i_x][y]=0;
                    }
                }
            }
        }

        fn merge(&mut self,x: usize,y: usize,command: Command) {
            let my_score = self.field[x][y];
            let (n_x,n_y,score) = self.neighbour(x,y,command);
            // is merge able
            if score>0 && n_x < self.length +1{
                if my_score == score {
                    //merge
                    self.field[x][y] = score+score;
                    //clear field
                    self.field[n_x][n_y] = 0;
                }
            }
        }

        fn neighbour(&self, x: usize,y: usize, command: Command) -> (usize,usize,u32){
            if command == Command::Up && y < self.length -1  {
                return (x,y+1,self.field[x][y+1]);
            }
            if command == Command::Down && y > 0 {
                return (x,y-1,self.field[x][y-1]);
            }
            if command == Command::Left && x < self.length -1 {
                return (x+1,y,self.field[x+1][y]);
            }
            if command == Command::Right && x > 0 {
                return (x-1,y,self.field[x-1][y]);
            }
            return (self.length +1,self.length +1,0)
        }

        fn direction(&self, command: Command) -> (i32,usize,usize,i32,usize,usize,bool){
            if command == Command::Up {
                return (1,0,self.length,1,0,self.length,true);
            }
            if command == Command::Down {
                return (1,0,self.length,-1,self.length,0,true);
            }
            if command == Command::Left {
                return (1,0,self.length,1,0,self.length,false);
            }
            if command == Command::Right {
                return (-1,self.length,0,1,0,self.length,true);
            }
            return (0,0,0,0,0,0,false)
        }

        pub fn render(&self) {
            println!("Score: {:6}", self.score);
            for r in 0..self.length {
                for c in 0..self.length {
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
            for c in 0..self.length {
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

            let mut j = 0;
            let mut free : Vec<(usize, usize)>;
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


        fn on_command(&mut self, cmd : Command) {
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

