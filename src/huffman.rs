struct Source {
    symbol: String,
    probability: f64
}

struct Node {
    symbol: Option<String>,
    probability: f64,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl Node {
    fn from_source(source: &Source) -> Self {
        Self {
            symbol: Some(source.symbol.clone()),
            probability: source.probability,
            left: None,
            right: None
        }
    }


}

fn encode(sources: &[Source]) -> Vec<u8> {
    let result: Vec<u8> = Vec::new();



    result
}

fn decode() {

}
