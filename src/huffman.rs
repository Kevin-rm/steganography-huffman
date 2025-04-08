use std::{
    cmp::Ordering,
    collections::HashMap
};

pub struct Source {
    pub symbol: String,
    pub probability: f64,
}

#[derive(Debug)]
pub struct Node {
    pub symbol: Option<String>,
    pub probability: f64,
    pub left:  Option<Box<Self>>, // 0
    pub right: Option<Box<Self>>, // 1
}

impl Node {
    pub fn from_source(source: &Source) -> Self {
        Self {
            symbol: Some(source.symbol.clone()),
            probability: source.probability,
            left:  None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
            && self.probability == other.probability
            && self.left  == other.left
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
pub fn build_tree(sources: &[Source]) -> Node {
    use std::collections::BinaryHeap;

    let merge = |n1: Node, n2: Node| -> Node {
        Node {
            symbol: None,
            probability: n1.probability + n2.probability,
            left:  Some(Box::new(n1)),
            right: Some(Box::new(n2)),
        }
    };

    let mut heap = BinaryHeap::new();
    for source in sources {
        heap.push(Node::from_source(source));
    }

    while heap.len() > 1 {
        let left  = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        heap.push(merge(left, right));
    }

    heap.pop().unwrap()
}

pub fn generate_codes(node: &Node) -> HashMap<String, Vec<u8>> {
    fn generate_codes(
        node: &Node,
        codes: &mut HashMap<String, Vec<u8>>,
        path: &mut Vec<u8>,
    ) {
        if node.is_leaf() {


            codes.insert(node.symbol.clone().unwrap(), path.clone());
            return;
        }

        for (child, bit) in [(&node.left, 0), (&node.right, 1)] {
            if let Some(child_node) = child {
                path.push(bit);
                generate_codes(child_node, codes, path);
                path.pop();
            }
        }
    }

    let mut codes = HashMap::new();
    generate_codes(node, &mut codes, &mut Vec::new());

    codes
}

fn encode(sources: &[Source]) -> Vec<u8> {
    let result: Vec<u8> = Vec::new();

    result
}

fn decode() {

}
