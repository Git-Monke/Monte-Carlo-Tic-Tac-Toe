use crate::sttt::{StrategicBoard, Move, ttt::MoveResult};
use rand::prelude::*;

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    data: Option<Move>,
    visits: f32,
    value: f32
}

pub struct Tree {
    pub root: Node,
    rng: ThreadRng
}

impl Node {
    fn new(mov: Option<Move>) -> Node {
        Node {
            children: vec![],
            data: mov,
            visits: 0.0,
            value: 0.0
        }
    }

    fn best_next_path(&self) -> usize {
        let mut best_index = 0;
        let mut highest_ucb = 0.0;

        for (i, child) in self.children.iter().enumerate() {
            let ucb = (child.value / child.visits) + (2.0 * (self.visits / child.visits).sqrt());
            if ucb > highest_ucb {
                best_index = i;
                highest_ucb = ucb;
            }
        }

        best_index
    }

    fn get_max_child(&self) -> &Node {
        &self.children.iter().max_by_key(|v| v.value as isize).unwrap()
    }

    fn get_min_child(&self) -> &Node {
        &self.children.iter().min_by_key(|v| v.value as isize).unwrap()
    }
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Node::new(None),
            rng: rand::thread_rng()
        }
    }

    pub fn step(&mut self, board: &mut StrategicBoard) {
        let mut path = vec![];
        let mut current_node = &mut self.root;
        board.set_checkpoint();

        loop {
            let len = current_node.children.len();

            match len {
                0 => break,
                _ => {
                    current_node.visits += 1.0;
                    let index = current_node.best_next_path();
                    current_node = &mut current_node.children[index];
                    
                    let mov = current_node.data.as_ref().unwrap();
                    board.make_move(mov.subboard, mov.index);

                    path.push(index);
                }
            }
        }

        if current_node.visits == 0.0 {
            let mut rollout_result = 0.0;
            current_node.visits += 1.0;

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
            
            let mut current_node = &mut self.root;
            current_node.value += rollout_result;

            for index in path.iter() {
                current_node = &mut current_node.children[*index];
                current_node.value += rollout_result;
            }
        } else {
            let moves = board.get_legal_moves();

            for mov in moves.iter() {
                current_node.children.push(Node::new(Some(Move {
                    subboard: mov.subboard,
                    index: mov.index
                })));
            }
        }
        
        board.revert();
    }
}