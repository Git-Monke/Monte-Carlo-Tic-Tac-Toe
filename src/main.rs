#[allow(dead_code, unused_variables, unused_imports)]
mod sttt;

use std::io;
use sttt::{ttt, StrategicBoard};
use ttt::{GameState, MoveResult};

fn read(prompt: &str) -> usize {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(num) if num >= 1 && num <= 9 => num,
        _ => {
            println!("Invalid input. Please enter a number between 1 and 9.");
            0
                }
    }
}

fn main() {
    let mut new_board = StrategicBoard::new();
    new_board.display();

    loop {
        let subboard = match new_board.current_board {
            Some(x) => {
                println!("Currently on board {}", x + 1);
                x
            },
            None => {
                read("Subboard (1-9): ") - 1
            }
        };
        let index = read("Index (1-9): ") - 1;

        let result = new_board.make_move(subboard, index);
        new_board.display();

        match result {
            MoveResult::PlayerWon(p) => {
                println!("Player {} won!", p);
                break;
            },
            MoveResult::Draw => {
                println!("Game was a draw!");
                break;
            },
            MoveResult::Nothing => ()
        }
    }
}
