use std::thread::sleep;
use std::time::Duration;
use crate::lib::chips::{o_chip, x_chip};
use crate::lib::game_result::GameResult;

pub struct Board {
    pub x_list: [i8; 9],
    pub o_list: [i8; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            x_list: [-1; 9], // -1 is a placeholder for empty cells
            o_list: [-1; 9],
        }
    }

    pub fn add_token(&mut self, cell: i8, x_turn: bool) -> bool {

        if cell < 1 || cell > 9 {
            println!("Please enter a number between 1 and 9");
            sleep(Duration::from_secs(3));
            return false
        }

        if self.x_list.contains(&cell) || self.o_list.contains(&cell) {
            println!("That cell is already taken");
            sleep(Duration::from_secs(3));
            return false
        }

        if x_turn {
            let place = cell - 1;
            self.x_list[place as usize] = cell;
        } else {
            let place = cell - 1;
            self.o_list[place as usize] = cell;
        }
        return true
    }
    pub fn check_winner(&self) -> GameResult {
        let winning_combinations: [[i8; 3]; 8] = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
            [1, 4, 7],
            [2, 5, 8],
            [3, 6, 9],
            [1, 5, 9],
            [3, 5, 7],
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
        // check for a tie
        let x_count = self.x_list.iter().filter(|&x| *x != -1).count();
        let y_count = self.o_list.iter().filter(|&x| *x != -1).count();
        if x_count + y_count == 9 {
            return GameResult::Tie
        }

        GameResult::InProgress
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn print_board(&self) {
        self.clear_screen();
        for i in 0..3 {
            for j in 0..3 {
                let current_cell = i * 3 + j + 1;
                if self.x_list.contains(&(current_cell)) {
                    print!(" {} ", x_chip());
                } else if self.o_list.contains(&(current_cell)) {
                    print!(" {} ", o_chip());
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
}