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

pub type Code = (u64, usize);

pub struct Tree {
    pub root: Node,
    pub codes: HashMap<String, Code>
}

impl Tree {
    fn generate_codes(root: &Node) -> HashMap<String, Code> {
        fn traverse_tree(
            node: &Node,
            current_code: u64,
            current_bit_length: usize,
            codes: &mut HashMap<String, Code>,
        ) {
            if node.is_leaf() {
                codes.insert(node.symbol.clone().unwrap(), (current_code, current_bit_length));
                return;
            }

            if let Some(left) = &node.left {
                traverse_tree(left, current_code << 1, current_bit_length + 1, codes);
            }

            if let Some(right) = &node.right {
                traverse_tree(right, (current_code << 1) | 1, current_bit_length + 1, codes);
            }
        }

        let mut codes = HashMap::new();
        traverse_tree(root, 0, 0, &mut codes);

        codes
    }

    pub fn build(sources: &[Source]) -> Self {
        use std::collections::BinaryHeap;

        let merge = |n1: Node, n2: Node| -> Node {
            Node {
                symbol: None,
                probability: n1.probability + n2.probability,
                left:  Some(Box::new(n1)),
                right: Some(Box::new(n2)),
            }
        };

        let mut heap = sources.iter()
            .map(Node::from_source)
            .collect::<BinaryHeap<_>>();

        while heap.len() > 1 {
            let left  = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            heap.push(merge(left, right));
        }

        let root = heap.pop().unwrap();
        let codes = Self::generate_codes(&root);
        Self { root, codes }
    }
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
        self.symbol.is_some() && self.left.is_none() && self.right.is_none()
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
