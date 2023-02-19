pub mod ttt;

use ttt::{Board, MoveResult};
use rand::prelude::*;

#[derive(Debug)]
struct Checkpoint {
    player: i8,
    current_board: Option<usize>,
    moves: Vec<usize>
}

#[derive(Debug)]
pub struct StrategicBoard {
    pub subboards: Vec<Board>,
    board: Board,
    pub legal_boards: Vec<usize>,
    pub current_board: Option<usize>,
    pub player: i8,
    pub in_play: bool,
    pub winner: i8,
    checkpoint: Checkpoint,
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
            in_play: true,
            winner: 0,
            checkpoint: Checkpoint {
                player: 1,
                current_board: None,
                moves: vec![]
            },
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
        if self.subboards[subboard].in_play == false {
            println!("Warning! Tried to play on subboard that has already been completed.");
            return MoveResult::Nothing;
        }

        // Check to make sure the spot they want to play on is empty
        if self.subboards[subboard].board[index] != 0 {
            println!("Warning! Tried to play on a spot that has already been played on.");
            return MoveResult::Nothing;
        }

        self.checkpoint.moves.push(subboard);

        let result = self.subboards[subboard].make_move(index, self.player);

        self.player = -self.player;

        self.current_board = match self.subboards[index].in_play {
            false => None,
            _ => Some(index),
        };
        
        match result {
            MoveResult::Completed(p) => {
                // Remove the board that is no longer in play from the list of legal boards
                if let Some(index) = self.legal_boards.iter().position(|&x| x == subboard) {
                    self.legal_boards.remove(index);
                }
                // Update the larger board to track that win, setting it to whoever won or 2 if the game was a draw.
                let result = self.board.make_move(subboard, p);

                match result {
                    MoveResult::Completed(p) => {
                        self.in_play = false;
                        self.winner = p;
                    },
                    _ => ()
                };
                result
            },
            MoveResult::Error => {
                panic!("Error was encountered");
            },
            _ => MoveResult::Nothing
        }
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        let subboards = if let Some(n) = self.current_board {
            vec![n]
        } else {
            self.subboards
                .iter()
                .enumerate()
                .filter_map(|(i, board)| if board.in_play == true { Some(i) } else { None })
                .collect()
        };

        for (i, board) in subboards.iter().enumerate() {
            let sub_moves = self.subboards[*board].get_legal_moves();

            for mov in sub_moves {
                moves.push(Move {
                    subboard: *board,
                    index: mov
                })
            }
        }

        moves
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
        self.checkpoint.player = self.player;
        self.checkpoint.current_board = self.current_board.clone();
        self.checkpoint.moves = vec![];
    }

    pub fn revert(&mut self) {
        for mov in self.checkpoint.moves.iter() {
            let subboard = &mut self.subboards[*mov];

            // When reverting a move, if the board was previously not in play that means it will become in play
            // This means we can add it to the list of legal boards
            if subboard.in_play == false {
                self.board.undo_move();
                self.legal_boards.push(*mov);
            }

            subboard.undo_move();
        }

        self.player = self.checkpoint.player;
        self.current_board = self.checkpoint.current_board.clone();
        self.winner = 0;
        self.in_play = true;
        self.checkpoint.moves = vec![];
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
