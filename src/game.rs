pub mod game_2048 {

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
            let field = vec![vec![0; length]; length];

            Game {
                length: length,
                score: 0,
                field: field
            }
        }

        pub fn score(&self) -> u32 {
            self.score
        }

        pub fn command(&mut self, command: Command) {
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
            for r in 0..self.length {
                for c in 0..self.length {
                    print!(" {} ", self.field[r][c])
                }
                println!("\n")
            }
        }
    }
}

