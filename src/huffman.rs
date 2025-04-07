use std::{
    cmp::Ordering,
    collections::HashMap
};

struct Source {
    symbol: String,
    probability: f64,
}

struct Node {
    symbol: Option<String>,
    probability: f64,
    left:  Option<Box<Self>>, // 0
    right: Option<Box<Self>>, // 1
}

impl Node {
    fn from_source(source: &Source) -> Self {
        Self {
            symbol: Some(source.symbol.clone()),
            probability: source.probability,
            left:  None,
            right: None,
        }
    }
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
            && self.probability == other.probability
            && self.left == other.left
            && self.right == other.right
    }
}

impl Eq for Node { }

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.probability.partial_cmp(&self.probability)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Returns the root
fn build_tree(sources: &[Source]) -> Node {
    use std::collections::BinaryHeap;

    let merge = |left: Node, right: Node| -> Node {
        Node {
            symbol: None,
            probability: left.probability + right.probability,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    };

    let mut heap = BinaryHeap::new();
    for source in sources {
        heap.push(Node::from_source(source));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        heap.push(merge(left, right));
    }

    heap.pop().unwrap()
}

fn generate_codes(node: &Node, codes: &mut HashMap<String, u8>) {

}

fn encode(sources: &[Source]) -> Vec<u8> {
    let result: Vec<u8> = Vec::new();

    result
}

fn decode() {

}
