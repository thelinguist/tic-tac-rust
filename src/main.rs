use crate::lib::state::State;
use crate::lib::game::Game;

mod lib;

use std::io;

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
fn main() {
    let quit = "q";
    let mut user_input = String::new();
    let mut state = State::StartGame;

    while !matches!(state, State::EndGame) {
        match state {
            State::StartGame => {
                clear_screen();
                println!("Welcome to Tic Tac Toe!");
                println!("Press {} to quit", quit);
                println!("Press any other key to continue");
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");
                if user_input.trim() == quit {
                    println!("Goodbye!");
                    break;
                }
                state = State::Play;
                println!("Starting game...");
            }
            State::EndGame => {
                println!("Game over!");
                break;
            }
            State::Play => {
                let mut game = Game::new();
                state = game.play();
            }
        }
    }
}
