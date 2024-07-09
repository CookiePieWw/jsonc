use crate::value::{Jsonc, Node};

pub fn decode(json: &Jsonc) -> String {
    let mut result = String::new();
    let iter = json.nodes.iter();
    let mut iter_str = json.strings.iter();
    let mut iter_num = json.numbers.iter();
    for node in iter {
        match node {
            Node::StartArray => {
                result.push('[');
            }
            Node::EndArray => {
                result.pop();
                result.push_str("],");
            }
            Node::StartObject => {
                result.push('{');
            }
            Node::EndObject => {
                result.pop();
                result.push_str("},");
            }
            Node::Key => {
                let key = iter_str.next().unwrap();
                result.push_str(&format!("\"{}\":", key));
            }
            Node::String => {
                let string = iter_str.next().unwrap();
                result.push_str(&format!("\"{}\",", string));
            }
            Node::Number => {
                let number = iter_num.next().unwrap();
                result.push_str(&format!("{},", number));
            }
            Node::True => {
                result.push_str("true,");
            }
            Node::False => {
                result.push_str("false,");
            }
            _ => {}
        }
    }
    result.pop();
    result
}
