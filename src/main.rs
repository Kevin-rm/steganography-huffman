use crate::huffman::{build_tree, generate_codes, Source};

mod huffman;

fn main() {
    let sources = vec![
        Source {
            symbol: String::from("s1"),
            probability: 0.3,
        },
        Source {
            symbol: String::from("s2"),
            probability: 0.2,
        },
        Source {
            symbol: String::from("s3"),
            probability: 0.15,
        },
        Source {
            symbol: String::from("s4"),
            probability: 0.15,
        },
        Source {
            symbol: String::from("s5"),
            probability: 0.10,
        },
        Source {
            symbol: String::from("s6"),
            probability: 0.10,
        },
    ];

    let root = build_tree(&sources);
    let generate_codes = generate_codes(&root);

    println!("{:?}", generate_codes)
}
