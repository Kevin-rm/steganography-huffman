use crate::huffman::{Source, Tree};

mod huffman;

fn main() {
    let sources = vec![
        Source::new(String::from("s1"), 0.3),
        Source::new(String::from("s2"), 0.2),
        Source::new(String::from("s3"), 0.15),
        Source::new(String::from("s4"), 0.15),
        Source::new(String::from("s5"), 0.10),
        Source::new(String::from("s6"), 0.10),
    ];

    let tree = Tree::build(&sources);
    print!("{:?}", tree.codes())
}
