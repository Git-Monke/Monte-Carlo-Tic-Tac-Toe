pub mod ttt;

use ttt::{Board, GameState, GameState::*, MoveResult};
use rand::prelude::*;

#[derive(Debug)]
pub struct StrategicBoard {
    pub subboards: Vec<Board>,
    board: Board,
    pub legal_boards: Vec<usize>,
    pub current_board: Option<usize>,
    pub player: i8,
    move_history: Vec<usize>,
    checkpoint_index: usize,
    rng: ThreadRng
}

#[derive(Debug)]
pub struct Move {
    pub subboard: usize,
    pub index: usize,
}

impl StrategicBoard {
    pub fn new() -> StrategicBoard {
        StrategicBoard {
            subboards: (0..9).map(|_| Board::new()).collect(),
            board: Board::new(),
            legal_boards: (0..9).map(|x| x).collect(),
            current_board: None,
            player: 1,
            move_history: vec![],
            checkpoint_index: 0,
            rng: rand::thread_rng()
        }
    }

    pub fn make_move(&mut self, subboard: usize, index: usize) -> MoveResult {
        // Check to make sure they're playing on the right board
        match self.current_board {
            Some(b) if b != subboard => {
                println!("Warning! Tried to play on the wrong board");
                return MoveResult::Nothing;
            }
            _ => (),
        }

        // Check to make sure the board is in play
        match self.subboards[subboard].state {
            Completed => {
                println!("Warning! Tried to play on subboard that has already been completed.");
                return MoveResult::Nothing;
            }
            InPlay => (),
        }

        // Check to make sure the spot they want to play on  is empty
        if self.subboards[subboard].board[index] != 0 {
            println!("Warning! Tried to play on a spot that has already been played on.");
            return MoveResult::Nothing;
        }

        let result = self.subboards[subboard].make_move(index, self.player);
        self.player = -self.player;
        self.move_history.push(subboard);

        self.current_board = match self.subboards[index].state {
            GameState::Completed => None,
            _ => Some(index),
        };

        match result {
            MoveResult::Completed(p) => {
                // Remove the board that is no longer in play from the list of legal boards
                if let Some(index) = self.legal_boards.iter().position(|&x| x == subboard) {
                    self.legal_boards.remove(index);
                }
                // Update the larger board to track that win, setting it to whoever won or 2 if the game was a draw.
                self.board.make_move(subboard, p)
            },
            _ => MoveResult::Nothing
        }
    }

    pub fn get_random_move(&mut self) -> Move {
        let subboard = match self.current_board {
            Some(b) => b,
            None => *self.legal_boards.choose(&mut self.rng).unwrap()
        };

        let index = self.subboards[subboard].get_random_move();

        Move {
            subboard: subboard,
            index: index
        }
    }

    pub fn set_checkpoint(&mut self) {
        self.checkpoint_index = self.move_history.len();
    }

    pub fn revert(&mut self) {
        while self.move_history.len() != self.checkpoint_index && self.move_history.len() >= 0 {
            let index = *self.move_history.last().unwrap();
            let subboard = &mut self.subboards[index];

            // When reverting a move, if the board was previously not in play that means it will become in play
            // This means we can add it to the list of legal boards
            if subboard.state != GameState::InPlay {
                self.legal_boards.push(index);
            }

            subboard.undo_move();
            self.move_history.pop();
        }
    }

    // Horrendous I know... but it works.
    pub fn display(&self) {
        for i in (0..=6).step_by(3) {
            for j in (0..=6).step_by(3) {
                println!(
                    " {}{}{} | {}{}{} | {}{}{} ",
                    match_token(self.subboards[i].board[j]),
                    match_token(self.subboards[i].board[j + 1]),
                    match_token(self.subboards[i].board[j + 2]),
                    match_token(self.subboards[i + 1].board[j]),
                    match_token(self.subboards[i + 1].board[j + 1]),
                    match_token(self.subboards[i + 1].board[j + 2]),
                    match_token(self.subboards[i + 2].board[j]),
                    match_token(self.subboards[i + 2].board[j + 1]),
                    match_token(self.subboards[i + 2].board[j + 2])
                );
            }
            if i < 6 {
                println!("-----+-----+-----")
            }
        }
    }
}

fn match_token(token: i8) -> &'static str {
    match token {
        1 => "X",
        -1 => "O",
        _ => ".",
    }
}
