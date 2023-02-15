pub enum MoveResult {
    PlayerWon(i8),
    Draw,
    Nothing,
}

#[derive(Debug)]
pub enum GameState {
    Winner,
    Draw,
    InPlay,
}

pub struct Board {
    pub board: [i8; 9],
    pub winner: i8,
    pub state: GameState,
    pub move_history: Vec<usize>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [0; 9],
            winner: 0,
            state: GameState::InPlay,
            move_history: vec![],
        }
    }

    // If player = 2, then that represents a claimed but neutral territory.
    // The purpose of this being to represent draws on bigger boards.
    pub fn make_move(&mut self, index: usize, player: i8) -> MoveResult {
        if self.winner != 0 {
            println!("Warning! Tried playing illegal move (Board already has a winner)");
            return MoveResult::Nothing;
        }

        if self.board[index] != 0 {
            println!("Warning! Tried playing illegal move (Player has already played at position)");
            return MoveResult::Nothing;
        }

        self.board[index] = player;
        self.move_history.push(index);
        self.check_for_win()
    }

    pub fn undo_move(&mut self) {
        self.board[*self.move_history.last().unwrap()] = 0;
        self.move_history.pop();
        self.state = GameState::InPlay;
    }

    pub fn get_legal_moves(&self) -> Vec<usize> {
        let mut moves = vec![];

        for (index, &value) in self.board.iter().enumerate() {
            if value == 0 {
                moves.push(index)
            }
        }

        moves
    }

    fn check_for_win(&mut self) -> MoveResult {
        let board = self.board;
        
        // Diagonal Checks
        if board[0] == board[4] && board[4] == board[8] && board[0] != 2 && board[0] != 0 {
            self.winner = board[0];
        }

        if board[2] == board[4] && board[4] == board[6] && board[0] != 2 && board[0] != 0 {
            self.winner = board[2];
        }

        // Horizontal Checks
        for index in 0..=2 {
            if board[index] == board[index + 3]
                && board[index] == board[index + 6]
                && board[index] != 2
                && board[index] != 0
            {
                self.winner = board[index];
            }
        }

        // Vertical Checks
        for index in (0..=6).step_by(3) {
            if board[index] == board[index + 1]
                && board[index] == board[index + 2]
                && board[index] != 2
                && board[index] != 0
            {
                self.winner = board[index];
            }
        }
        
        if self.winner != 0 {
            self.state = GameState::Winner;
            return MoveResult::PlayerWon(self.winner);
        }

        // If there are no 0's and the board is completely full, it's a draw
        if self.board.iter().any(|&x| x == 0) == false {
            self.state = GameState::Draw;
            return MoveResult::Draw;
        }

        MoveResult::Nothing
    }

    // pub fn display(&self) {
    //     println!(
    //         " {} | {} | {}",
    //         match_token(self.board[0]),
    //         match_token(self.board[1]),
    //         match_token(self.board[2])
    //     );
    //     println!("---+---+---");
    //     println!(
    //         " {} | {} | {}",
    //         match_token(self.board[3]),
    //         match_token(self.board[4]),
    //         match_token(self.board[5])
    //     );
    //     println!("---+---+---");
    //     println!(
    //         " {} | {} | {}",
    //         match_token(self.board[6]),
    //         match_token(self.board[7]),
    //         match_token(self.board[8])
    //     );
    // }
}

fn match_token(token: i8) -> &'static str {
    match token {
        1 => "X",
        -1 => "O",
        _ => " ",
    }
}
