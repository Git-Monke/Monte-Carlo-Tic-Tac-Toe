use crate::sttt::{StrategicBoard, Move, ttt::MoveResult};

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub data: Option<Move>,
    pub visits: f32,
    pub value: f32,
    terminal: bool
}

#[derive(Debug)]
pub struct Tree {
    pub root: Node,
    pub player: f32
}

impl Node {
    fn new(mov: Option<Move>) -> Node {
        Node {
            children: vec![],
            data: mov,
            visits: 0.0,
            value: 0.0,
            terminal: false
        }
    }

    fn best_next_path(&self) -> usize {
        let mut best_index = 0;
        let mut highest_ucb = 0.0;

        for (i, child) in self.children.iter().enumerate() {
            if child.terminal {
                continue;
            }

            let mut ucb = (child.value / child.visits) + (1.0 * (self.visits.ln() / child.visits).sqrt());
            if ucb.is_nan() { ucb = f32::INFINITY };

            if ucb > highest_ucb {
                best_index = i;
                highest_ucb = ucb;
            }
        }

        best_index
    }

    pub fn get_max_child(&self) -> &Node {
        &self.children.iter().max_by_key(|v| v.value as isize).unwrap()
    }

    pub fn get_min_child(&self) -> &Node {
        &self.children.iter().min_by_key(|v| v.value as isize).unwrap()
    }
}

impl Tree {
    pub fn new(player: f32) -> Tree {
        Tree {
            root: Node::new(None),
            player: player
        }
    }

    pub fn step(&mut self, board: &mut StrategicBoard) -> u32 {
        let mut path = vec![];
        let mut current_node = &mut self.root;
        let mut depth = 1;

        board.set_checkpoint();

        loop {
            let len = current_node.children.len();

            match len {
                0 => break,
                _ => {
                    depth += 1;
                    let index = current_node.best_next_path();
                    current_node = &mut current_node.children[index];
                    
                    let mov = current_node.data.as_ref().unwrap();
                    board.make_move(mov.subboard, mov.index);

                    path.push(index);
                }
            }
        }
        
        if current_node.visits != 0.0 {
            let moves = board.get_legal_moves();

            for mov in moves.iter() {
                current_node.children.push(Node::new(Some(Move {
                    subboard: mov.subboard,
                    index: mov.index
                })));
            }

            depth += 1;
            board.make_move(moves[0].subboard, moves[0].index);
        }

        let mut rollout_result: f32;

        // If board is terminal by default, just use who won
        // Otherwise roll the game out
        if board.in_play == false {
            rollout_result = board.winner as f32;
            current_node.terminal = true;
        } else {
            loop {
                let mov = board.get_random_move();
                let result = board.make_move(mov.subboard, mov.index);
    
                match result {
                    MoveResult::Completed(p) => {
                        rollout_result = match p {
                            1 | -1 => p as f32,
                            2 => 0.0,
                            _ => unreachable!()
                        };
                        break
                    },
                    _ => ()
                }
            }
        }
        
        rollout_result *= self.player;

        let mut current_node = &mut self.root;
        current_node.value += rollout_result;
        current_node.visits += 1.0;

        for index in path.iter() {
            current_node = &mut current_node.children[*index];
            current_node.value += rollout_result;
            current_node.visits += 1.0;
        }
        
        board.revert();
        depth
    }
}