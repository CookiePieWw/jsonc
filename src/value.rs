use crate::{decoder::decode_slice, slice::JsoncSlice};

#[derive(Debug, PartialEq, Clone)]
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

impl From<&Node> for u8 {
    fn from(node: &Node) -> u8 {
        match node {
            Node::Null => 0,
            Node::StartArray => 1,
            Node::EndArray => 2,
            Node::StartObject => 3,
            Node::EndObject => 4,
            Node::Key => 5,
            Node::String => 6,
            Node::Number => 7,
            Node::True => 8,
            Node::False => 9,
        }
    }
}

impl From<&u8> for Node {
    fn from(n: &u8) -> Node {
        match n {
            0 => Node::Null,
            1 => Node::StartArray,
            2 => Node::EndArray,
            3 => Node::StartObject,
            4 => Node::EndObject,
            5 => Node::Key,
            6 => Node::String,
            7 => Node::Number,
            8 => Node::True,
            9 => Node::False,
            _ => panic!("Invalid node value"),
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
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

    pub fn new_with_values(nodes: Vec<Node>, strings: Vec<String>, numbers: Vec<f64>) -> Self {
        Self {
            nodes,
            strings,
            numbers,
        }
    }

    pub fn append(&mut self, other: &mut Jsonc) {
        self.nodes.append(&mut other.nodes);
        self.strings.append(&mut other.strings);
        self.numbers.append(&mut other.numbers);
    }

    pub fn node_opt_list(&self) -> Vec<Option<u8>> {
        let mut node_list = Vec::new();
        for node in self.nodes.iter() {
            node_list.push(Some(node.into()));
        }
        node_list
    }

    pub fn string_opt_list(&self) -> Vec<Option<String>> {
        self.strings.clone().into_iter().map(Some).collect()
    }

    pub fn number_opt_list(&self) -> Vec<Option<f64>> {
        self.numbers.clone().into_iter().map(Some).collect()
    }

    pub fn as_slice(&self) -> JsoncSlice {
        self.into()
    }

    pub fn get(&self, paths: &[&str]) -> Option<String> {
        let mut json_slice = self.as_slice();
        for path in paths {
            if path.starts_with("\"") {
                if let Some(slice) = json_slice.get_by_path(&path[1..path.len() - 1]) {
                    json_slice = slice;
                } else {
                    return None;
                }
            } else {
                let idx = path.parse::<usize>().unwrap();
                if let Some(slice) = json_slice.get_by_idx(idx) {
                    json_slice = slice;
                } else {
                    return None;
                }
            }
        }
        Some(decode_slice(json_slice))
    }
}
