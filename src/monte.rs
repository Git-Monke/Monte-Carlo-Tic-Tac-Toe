// This is the exploration constant.
// The higher this value is, the more likely the tree will be to search new paths.
// Lower values will spend more time searching already good paths.
let exploration_constant = 2.0;

pub struct Node<T> {
    parent: Option<Box<Node>>,
    children: Vec<Node>,
    node_data: Option<T>
    value: f32,
    visits: f32
}

impl Node {
    pub fn new(parent: Option<Box<Node>>) -> Node<T> {
        Node {
            parent: parent,
            leaves: vec![],
            node_data: None,
            value: 0,
            visits: 0
        }
    }

    // Propogates a value up the tree.
    pub fn propogate_value(&mut self, value: isize) {
        self.value += value;
        self.visits += 1;

        if self.parent.is_some() {
            self.parent.propogate_value(-value);
        }
    }

    pub fn get_ucb(&self) -> f32 {
        if (self.parent.is_none()) {
            return 0.0
        }

        (self.value / self.visits) + (exploration_constant * (self.parent.visits / self.visits).sqrt())
    }
}