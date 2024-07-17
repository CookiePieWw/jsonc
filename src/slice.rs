use crate::value::{Jsonc, Node};

#[derive(Debug, PartialEq)]
pub struct JsoncSlice<'a> {
    pub nodes: &'a [Node],
    pub strings: &'a [String],
    pub numbers: &'a [f64],
}

impl<'a> JsoncSlice<'a> {
    pub fn new(jsonc: &'a Jsonc) -> Self {
        Self {
            nodes: &jsonc.nodes,
            strings: &jsonc.strings,
            numbers: &jsonc.numbers,
        }
    }
}

impl<'a> JsoncSlice<'a> {
    pub fn get_by_path(&self, path: &str) -> Option<JsoncSlice<'a>> {
        if self.nodes.is_empty() || self.nodes[0] != Node::StartObject {
            return None;
        }
        let mut str_idx = 0;
        let mut num_idx = 0;
        let mut nest = 0;
        for (node_idx, node) in self.nodes.iter().enumerate() {
            if nest == -1 {
                return None;
            }
            if node_idx == 0 {
                continue;
            }
            match node {
                Node::Key => {
                    if nest == 0 {
                        let key = self.strings.get(str_idx).unwrap();
                        if key == path {
                            return Some(self.strip_slice(node_idx + 1, num_idx, str_idx + 1));
                        }
                    }
                    str_idx += 1;
                }
                Node::String => {
                    str_idx += 1;
                }
                Node::Number => {
                    num_idx += 1;
                }
                Node::StartArray | Node::StartObject => {
                    nest += 1;
                }
                Node::EndArray | Node::EndObject => {
                    nest -= 1;
                }
                _ => {}
            }
        }
        return None;
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<JsoncSlice<'a>> {
        if self.nodes.is_empty() || self.nodes[0] != Node::StartArray {
            return None;
        }
        let mut count = 0;
        let mut str_idx = 0;
        let mut num_idx = 0;
        let mut nest = 0;
        for (node_idx, node) in self.nodes.iter().enumerate() {
            if nest == -1 {
                return None;
            }
            if node_idx == 0 {
                continue;
            }
            if nest == 0 && count == idx {
                return Some(self.strip_slice(node_idx, num_idx, str_idx));
            }
            match node {
                Node::String => {
                    count += 1;
                    str_idx += 1;
                }
                Node::Number => {
                    count += 1;
                    num_idx += 1;
                }
                Node::StartArray | Node::StartObject => {
                    if nest == 0 {
                        count += 1;
                    }
                    nest += 1;
                }
                Node::EndArray | Node::EndObject => {
                    nest -= 1;
                }
                _ => {
                    count += 1;
                }
            }
        }
        return None;
    }

    fn strip_slice(&self, node_start: usize, num_start: usize, str_start: usize) -> JsoncSlice<'a> {
        match self.nodes[node_start] {
            Node::StartArray | Node::StartObject => {
                let mut nest = 1;
                let mut node_end = node_start + 1;
                let mut num_end = num_start;
                let mut str_end = str_start;
                while nest > 0 {
                    match self.nodes[node_end] {
                        Node::StartArray | Node::StartObject => {
                            nest += 1;
                        }
                        Node::EndArray | Node::EndObject => {
                            nest -= 1;
                        }
                        Node::String => {
                            str_end += 1;
                        }
                        Node::Number => {
                            num_end += 1;
                        }
                        Node::Key => {
                            str_end += 1;
                        }
                        _ => {}
                    }
                    node_end += 1;
                }
                JsoncSlice {
                    nodes: &self.nodes[node_start..node_end],
                    strings: &self.strings[str_start..str_end],
                    numbers: &self.numbers[num_start..num_end],
                }
            }
            Node::String => {
                JsoncSlice {
                    nodes: &self.nodes[node_start..node_start + 1],
                    strings: &self.strings[str_start..str_start + 1],
                    numbers: &self.numbers[num_start..num_start],
                }
            }
            Node::Number => {
                JsoncSlice {
                    nodes: &self.nodes[node_start..node_start + 1],
                    strings: &self.strings[str_start..str_start],
                    numbers: &self.numbers[num_start..num_start + 1],
                }
            }
            _ => JsoncSlice {
                nodes: &self.nodes[node_start..node_start + 1],
                strings: &self.strings[str_start..str_start],
                numbers: &self.numbers[num_start..num_start],
            },
        }
    }
}

impl<'a> From<&'a Jsonc> for JsoncSlice<'a> {
    fn from(jsonc: &'a Jsonc) -> JsoncSlice<'a> {
        JsoncSlice::new(jsonc)
    }
}

impl From<JsoncSlice<'_> > for Jsonc {
    fn from(jsonc_slice: JsoncSlice) -> Jsonc {
        Jsonc {
            nodes: jsonc_slice.nodes.to_vec(),
            strings: jsonc_slice.strings.to_vec(),
            numbers: jsonc_slice.numbers.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonc_get() {
        let jsonc = Jsonc {
            nodes: vec![
                Node::StartObject,
                Node::Key,
                Node::String,
                Node::Key,
                Node::String,
                Node::EndObject,
            ],
            strings: vec![
                "key1".to_string(),
                "value1".to_string(),
                "key2".to_string(),
                "value2".to_string(),
            ],
            numbers: vec![],
        };
        let jsonc_slice = JsoncSlice::new(&jsonc);
        let result = jsonc_slice.get_by_path("key1");
        assert_eq!(result, Some(JsoncSlice {
            nodes: &[Node::String],
            strings: &["value1".to_string()],
            numbers: &[],
        }));
        let result = jsonc_slice.get_by_path("key2");
        assert_eq!(result, Some(JsoncSlice {
            nodes: &[Node::String],
            strings: &["value2".to_string()],
            numbers: &[],
        }));
        let result = jsonc_slice.get_by_path("key3");
        assert_eq!(result.is_none(), true);
    }
}
