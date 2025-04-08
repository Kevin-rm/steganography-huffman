use crate::huffman::{Source, Tree};

mod huffman;

fn main() {
    let sources = vec![
        Source::new("s1", 0.3),
        Source::new("s2", 0.2),
        Source::new("s3", 0.15),
        Source::new("s4", 0.15),
        Source::new("s5", 0.10),
        Source::new("s6", 0.10),
    ];

    let tree = Tree::build(&sources);
    println!("{}", tree.render());
}
