pub mod ttt;

use ttt::{Board, GameState, GameState::*, MoveResult};

pub struct StrategicBoard {
    subboards: Vec<Board>,
    board: Board,
    pub current_board: Option<usize>,
    pub player: i8,
    move_history: Vec<usize>,
    checkpoint_index: usize,
}

impl StrategicBoard {
    pub fn new() -> StrategicBoard {
        StrategicBoard {
            subboards: (0..9).map(|_| Board::new()).collect(),
            board: Board::new(),
            current_board: None,
            player: 1,
            move_history: vec![],
            checkpoint_index: 0
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
            Winner | Draw => {
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
        
        self.current_board = match self.subboards[index].state {
            GameState::Winner | GameState::Draw => None,
            _ => Some(index),
        };

        match result {
            MoveResult::PlayerWon(p) => self.board.make_move(subboard, p),
            MoveResult::Draw => self.board.make_move(subboard, 2),
            _ => MoveResult::Nothing,
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
