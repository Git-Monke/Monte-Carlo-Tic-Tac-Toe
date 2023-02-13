mod ttt;

use std::io;
use ttt::{Board, MoveResult};

pub fn main() {
    let mut board = Board::new();
    let mut player: i8 = 1;
    
    board.display();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let position: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, expected 1-9");
                return;
            },
        };

        let result = board.make_move(position - 1, player);
        board.display();
        player = -player;

        match result {
            MoveResult::PlayerWon(p) => {
                println!("Winner is player {}", match board.winner { 1 => 1, -1 => 2, _ => 0 });
                break;
            }
            MoveResult::Draw => {
                println!("Game is a draw!");
                break;
            }
            MoveResult::Nothing => ()
        }
    }
}