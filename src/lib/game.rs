use crate::lib::state::State;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use crate::lib::board::Board;
use crate::lib::constants::QUIT;
use crate::lib::game_result::GameResult;

pub struct Game {
    pub board: Board,
    pub x_turn: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            x_turn: true,
        }
    }

    fn print_prompt(&self) {
        let turn = if self.x_turn { "X" } else { "O" };
        println!("{turn}: Enter a number between 1 and 9 to place your mark (or {QUIT} to quit)");
    }

    pub fn play(&mut self) -> State {
        let mut user_input = String::new();

        while user_input.trim() != QUIT {
            self.board.print_board();
            self.print_prompt();
            io::stdin()
                .read_line(&mut user_input).expect("Failed to read line");

            let cell: i8 = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    if (user_input.trim() == QUIT) {
                        println!("Goodbye!");
                        return State::EndGame
                    }
                    println!("Please enter a valid number");
                    sleep(Duration::from_secs(3));
                    user_input.clear();
                    continue;
                }
            };
            let is_valid = self.board.add_token(cell, self.x_turn);
            if !is_valid {
                user_input.clear();
                continue;
            }
            self.x_turn = !self.x_turn;

            let result = self.board.check_winner();

            self.board.print_board();
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