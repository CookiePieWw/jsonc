#[derive(Debug, PartialEq)]
pub enum Node {
    Null,
    StartArray,
    EndArray,
    StartObject,
    EndObject,
    Key,
    String,
    Number,
    True,
    False,
}

#[derive(Debug, PartialEq, Default)]
pub struct Jsonc {
    pub nodes: Vec<Node>,
    pub strings: Vec<String>,
    pub numbers: Vec<f64>,
}

impl Jsonc {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            strings: Vec::new(),
            numbers: Vec::new(),
        }
    }
}
