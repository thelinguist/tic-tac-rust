use crate::lib::state::State;
use std::io;
use std::thread::sleep;
use colored::Colorize;

pub struct Game {
    pub x_list: Vec<i8>,
    pub o_list: Vec<i8>,
    pub x_turn: bool,
}

enum GameResult {
    XWon,
    OWon,
    Tie,
    InProgress,
}

impl Game {
    pub fn new() -> Game {
        Game {
            x_list: Vec::new(),
            o_list: Vec::new(),
            x_turn: true,
        }
    }
    pub fn check_winner(&self) -> GameResult {
        let winning_combinations: Vec<Vec<i8>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
            vec![1, 5, 9],
            vec![3, 5, 7],
        ];
        for combination in winning_combinations {
            let mut x_count = 0;
            let mut o_count = 0;
            for cell in combination {
                if self.x_list.contains(&cell) {
                    x_count += 1;
                }
                if self.o_list.contains(&cell) {
                    o_count += 1;
                }
            }
            if x_count == 3 {
                return GameResult::XWon
            }
            if o_count == 3 {
                return GameResult::OWon
            }
        }
        if (self.x_list.len() + self.o_list.len()) == 9 {
            return GameResult::Tie
        }
        GameResult::InProgress
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn print_prompt(&self) {
        let turn = if self.x_turn { "X" } else { "O" };
        println!("{turn}: Enter a number between 1 and 9 to place your mark (or q to quit)");
    }


    fn print_board(&self) {
        let x_color = "X".blue().bold();
        let o_color = "O".magenta().bold();
        self.clear_screen();
        for i in 0..3 {
            for j in 0..3 {
                let current_cell = i * 3 + j + 1;
                if self.x_list.contains(&(current_cell)) {
                    print!(" {} ", x_color);
                } else if self.o_list.contains(&(current_cell)) {
                    print!(" {} ", o_color);
                } else {
                    print!(" {} ", current_cell);
                }
                if j < 2 {
                    print!("|");
                }
            }
            print!("\n");
            if i < 2 {
                println!("---------")
            }
        }
    }

    pub fn play(&mut self) -> State {
        let mut user_input = String::new();

        while user_input.trim() != "q" {
            self.print_board();
            self.print_prompt();
            io::stdin()
                .read_line(&mut user_input).expect("Failed to read line");

            let cell: i8 = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number");
                    sleep(std::time::Duration::from_secs(3));
                    user_input.clear();
                    continue;
                }
            };

            if cell < 1 || cell > 9 {
                println!("Please enter a number between 1 and 9");
                sleep(std::time::Duration::from_secs(3));
                user_input.clear();
                continue;
            }

            if self.x_list.contains(&cell) || self.o_list.contains(&cell) {
                println!("That cell is already taken");
                sleep(std::time::Duration::from_secs(3));
                user_input.clear();
                continue;
            }

            if self.x_turn {
                self.x_list.push(cell);
            } else {
                self.o_list.push(cell);
            }
            self.x_turn = !self.x_turn;

            let result = self.check_winner();

            self.print_board();
            match result {
                GameResult::XWon => {
                    println!("X won!");
                    return State::EndGame
                },
                GameResult::OWon => {
                    println!("O won!");
                    return State::EndGame
                },
                GameResult::Tie => {
                    println!("It's a tie!");
                    return State::EndGame
                },
                GameResult::InProgress => {
                    user_input.clear();
                },
            }
        }
        State::EndGame
    }
}