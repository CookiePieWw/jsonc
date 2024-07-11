use crate::value::{Jsonc, Node};
use jsonb::functions::escape_scalar_string;

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
                if result.ends_with(',') {
                    result.pop();
                }
                result.push_str("],");
            }
            Node::StartObject => {
                result.push('{');
            }
            Node::EndObject => {
                if result.ends_with(',') {
                    result.pop();
                }
                result.push_str("},");
            }
            Node::Key => {
                let key = iter_str.next().unwrap();
                result.push_str(&format!("\"{}\":", key));
            }
            Node::String => {
                let string = iter_str.next().unwrap();
                let bytes = string.as_bytes();
                let mut escaped_string = String::new();
                escape_scalar_string(bytes, 0, bytes.len(), &mut escaped_string);
                result.push_str(&escaped_string);
                result.push(',');
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
            Node::Null => {
                result.push_str("null,");
            }
        }
    }
    if result.ends_with(',') {
        result.pop();
    }
    result
}
