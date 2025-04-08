use std::{
    cmp::Ordering,
    collections::HashMap
};

pub struct Source {
    symbol: String,
    probability: f64,
}

impl Source {
    pub fn new(symbol: &str, probability: f64) -> Self {
        Source { symbol: symbol.to_string(), probability }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn probability(&self) -> f64 {
        self.probability
    }
}

pub type Code = (u64, usize);

pub struct Tree {
    root: Node,
    leaf_count: usize,
    codes: HashMap<String, Code>
}

const TEE:      &str = "├── ";
const ELBOW:    &str = "└── ";
const VERTICAL: &str = "│";
const SPACE:    &str = " ";

impl Tree {
    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn leaf_count(&self) -> usize {
        self.leaf_count
    }

    pub fn codes(&self) -> &HashMap<String, Code> {
        &self.codes
    }

    #[must_use]
    pub fn build(sources: &[Source]) -> Self {
        use std::collections::BinaryHeap;

        let merge_nodes = |n1: Node, n2: Node| -> Node {
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
            heap.push(merge_nodes(left, right));
        }

        let root = heap.pop().unwrap();
        let leaf_count = sources.len();
        let codes = Self::generate_codes(&root, leaf_count);
        Self { root, leaf_count, codes }
    }

    fn generate_codes(root: &Node, leaf_count: usize) -> HashMap<String, Code> {
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

        let mut codes = HashMap::with_capacity(leaf_count);
        traverse_tree(root, 0, 0, &mut codes);

        codes
    }

    pub fn render(&self) -> String {
        fn render_node(node: &Node, prefix: &str, is_left: bool, result: &mut String) {
            result.push_str(&format!("{}{}", prefix, if is_left { TEE } else { ELBOW }));
            
            if node.is_leaf() {
                result.push_str(&format!("\"{}\" (p={})\n",
                    node.symbol().unwrap(), node.probability()
                ));
                return;
            }
            result.push_str(&format!("Noeud (p={})\n",
                node.probability()
            ));
            
            let new_prefix = format!("{}{}   ", prefix, if is_left { VERTICAL } else { SPACE });
            if let Some(left) = &node.left() {
                render_node(left, &new_prefix, true, result);
            }
            
            if let Some(right) = &node.right() {
                render_node(right, &new_prefix, false, result);
            }
        }
        
        let mut result = String::from(&format!("Arbre d'Huffman (Nombre de feuilles: {})\n", self.leaf_count));
        render_node(&self.root, "", false, &mut result);

        result.push_str("\nTable de codage:\n");
        let mut codes: Vec<(&String, &Code)> = self.codes.iter().collect();
        codes.sort_by(|a, b| a.0.cmp(b.0));
        
        for (symbol, (code, bits)) in codes {
            let code_bits = format!("{:b}", code);
            result.push_str(&format!("\"{}\" => {}{} ({} bits)\n",
                symbol, "0".repeat(bits.saturating_sub(code_bits.len())), code_bits, bits
            ));
        }
        
        result
    }
}

#[derive(Debug)]
pub struct Node {
    symbol: Option<String>,
    probability: f64,
    left:  Option<Box<Self>>, // 0
    right: Option<Box<Self>>, // 1
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

    pub fn symbol(&self) -> Option<&str> {
        self.symbol.as_deref()
    }

    pub fn probability(&self) -> f64 {
        self.probability
    }

    pub fn left(&self) -> &Option<Box<Self>> {
        &self.left
    }

    pub fn right(&self) -> &Option<Box<Self>> {
        &self.right
    }
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol               &&
            self.probability == other.probability &&
            self.left  == other.left              &&
            self.right == other.right
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
        self.partial_cmp(other)
            .unwrap_or(Ordering::Equal)
    }
}
