use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt,
    fmt::{Display, Formatter}
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
    codes: HashMap<String, Code>,
    sorted_codes: Vec<(String, Code)>
}

const TEE:      &str = "├── ";
const ELBOW:    &str = "└── ";
const VERTICAL: &str = "│";
const SPACE:    &str = " ";

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.render(), self.render_code_tables())
    }
}

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
            .map(Node::from)
            .collect::<BinaryHeap<_>>();

        while heap.len() > 1 {
            let left  = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            heap.push(merge_nodes(left, right));
        }

        let root = heap.pop().unwrap();
        let leaf_count = sources.len();
        let codes = Self::generate_codes(&root, leaf_count);
        let mut sorted_codes: Vec<_> = codes.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        sorted_codes.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));

        Self { root, leaf_count, codes, sorted_codes }
    }

    pub fn encode(&self, input: &str) -> Result<(Vec<u8>, usize), Error> {
        let mut result = Vec::with_capacity(input.len() / 2);
        let mut current_byte = 0u8;
        let mut bit_position = 7;
        let mut total_bits = 0;

        let mut remaining_input = input;
        while !remaining_input.is_empty() {
            match self.sorted_codes.iter()
                .find(|(symbol, _)|
                    remaining_input.starts_with(symbol.as_str()
                )) {
                Some((symbol, (code, bits))) => {
                    for i in (0..*bits).rev() {
                        current_byte |= ((*code >> i & 1) as u8) << bit_position;

                        if bit_position == 0 {
                            result.push(current_byte);
                            current_byte = 0;
                            bit_position = 7;
                        } else { bit_position -= 1; }
                        total_bits += 1;
                    }
                    remaining_input = &remaining_input[symbol.len()..];
                }
                None => {
                    return Err(Error::SymbolNotFound(remaining_input.chars().next()
                        .map(|c| c.to_string())
                        .unwrap_or_default()
                    ));
                }
            }
        }

        if bit_position != 7 { result.push(current_byte); }

        Ok((result, total_bits))
    }

    pub fn decode(&self, encoded: &[u8], bit_len: usize) -> Result<String, Error> {
        let mut result = String::new();
        let mut current_node = &self.root;
        let mut bits_read = 0;

        for &byte in encoded {
            for i in (0..8).rev() {
                if bits_read >= bit_len { break; }

                current_node = match (byte >> i) & 1 {
                    0 => current_node.left.as_ref(),
                    1 => current_node.right.as_ref(),
                    _ => unreachable!(),
                }.ok_or(Error::InvalidCode)?;

                if !current_node.is_leaf() { 
                    bits_read += 1;
                    continue 
                }

                result.push_str(current_node.symbol().unwrap());
                current_node = &self.root;
                bits_read += 1;
            }
        }

        if current_node != &self.root && bits_read < bit_len { return Err(Error::InvalidCode); }

        Ok(result)
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
        
        result
    }

    pub fn render_code_tables(&self) -> String {
        let mut codes: Vec<(&String, &Code)> = self.codes.iter().collect();
        codes.sort_by(|a, b| a.0.cmp(b.0));

        let mut result = String::from("Table de codage:\n");
        for (symbol, (code, bits)) in codes {
            let code_bits = format!("{:b}", code);
            result.push_str(&format!("\"{}\" => {}{} ({} bits)\n",
                symbol, "0".repeat(bits.saturating_sub(code_bits.len())), code_bits, bits
            ));
        }

        result
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
}

#[derive(Debug)]
pub struct Node {
    symbol: Option<String>,
    probability: f64,
    left:  Option<Box<Self>>, // 0
    right: Option<Box<Self>>, // 1
}

impl From<&Source> for Node {
    fn from(value: &Source) -> Self {
        Self {
            symbol: Some(value.symbol.clone()),
            probability: value.probability,
            left:  None,
            right: None,
        }
    }
}

impl Node {
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

#[derive(Debug)]
pub enum Error { SymbolNotFound(String), InvalidCode }

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::SymbolNotFound(s) => write!(f, "Symbole \"{}\" non trouvé dans l'arbre", s),
            Error::InvalidCode               => write!(f, "Code binaire invalide"),
        }
    }
}

impl std::error::Error for Error { }
