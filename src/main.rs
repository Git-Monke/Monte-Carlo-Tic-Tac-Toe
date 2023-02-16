#[allow(dead_code, unused_variables, unused_imports)]
mod sttt;

use std::time::Instant;

use std::io;
use sttt::{ttt, StrategicBoard};
use ttt::{GameState, MoveResult};

fn read(prompt: &str) -> usize {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

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
    new_board.set_checkpoint();

    let now = Instant::now();

    for _ in 0..100 {
        new_board.revert();
        new_board.display();
        loop {
            let new_move = new_board.get_random_move();
            new_board.display();
            println!("\n");
            println!("{:?}", new_move);
            println!("{:?}", new_board.subboards);  
            new_board.subboards[new_move.subboard].display(); 
            let result = new_board.make_move(new_move.subboard, new_move.index);
            match result {
                MoveResult::Completed(p) => {
                    println!("{}", p);
                    break
                }
                _ => ()
            };
        }
    }

    println!("{:?}", now.elapsed());

    // loop {
    //     let subboard = match new_board.current_board {
    //         Some(x) => {
    //             println!("Currently on board {}", x + 1);
    //             x
    //         }
    //         None => read("Subboard (1-9): ") - 1,
    //     };
    //     let index = read("Index (1-9): ") - 1;

    //     let result = new_board.make_move(subboard, index);
    //     new_board.display();
    //     println!("{:?}", new_board.get_random_move());

    //     match result {
    //         MoveResult::Completed(p) => {
    //             match p {
    //                 1 => println!("Player 1 won!"),
    //                 -1 => println!("Player 2 won!"),
    //                 2 => println!("Game was a draw"),
    //                 _ => unreachable!(),
    //             };
    //             break;
    //         }
    //         MoveResult::Nothing => (),
    //     }
    // }
}
