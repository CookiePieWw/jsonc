use serde_json::{Number, Value};

use crate::value::{Jsonc, Node};

impl From<Value> for Jsonc {
    fn from(value: Value) -> Self {
        let mut jsonc = Jsonc::default();
        match value {
            Value::Null => {
                jsonc.nodes.push(Node::Null);
            }
            Value::Bool(true) => {
                jsonc.nodes.push(Node::True);
            }
            Value::Bool(false) => {
                jsonc.nodes.push(Node::False);
            }
            Value::Number(n) => {
                jsonc.nodes.push(Node::Number);
                jsonc.numbers.push(n.as_f64().unwrap());
            }
            Value::String(s) => {
                jsonc.nodes.push(Node::String);
                jsonc.strings.push(s);
            }
            Value::Array(arr) => {
                jsonc.nodes.push(Node::StartArray);
                for v in arr {
                    let mut jsonc_v = Jsonc::from(v);
                    jsonc.append(&mut jsonc_v);
                }
                jsonc.nodes.push(Node::EndArray);
            }
            Value::Object(obj) => {
                jsonc.nodes.push(Node::StartObject);
                for (k, v) in obj {
                    jsonc.nodes.push(Node::Key);
                    jsonc.strings.push(k);
                    let mut jsonc_v = Jsonc::from(v);
                    jsonc.append(&mut jsonc_v);
                }
                jsonc.nodes.push(Node::EndObject);
            }
        }

        jsonc
    }
}

/// Returns value, the number of nodes consumed, strings consumed, and numbers consumed
fn jsonc_list_to_serde_value(
    nodes: &[Node],
    strings: &[String],
    numbers: &[f64],
) -> (Value, usize, usize, usize) {
    match nodes[0] {
        Node::Null => (Value::Null, 1, 0, 0),
        Node::True => (Value::Bool(true), 1, 0, 0),
        Node::False => (Value::Bool(false), 1, 0, 0),
        Node::Number => (
            Value::Number(Number::from_f64(numbers[0]).unwrap()),
            1,
            0,
            1,
        ),
        Node::String => (Value::String(strings[0].clone()), 1, 1, 0),
        Node::StartArray => {
            let mut arr = Vec::new();
            let mut node_idx = 1;
            let mut string_idx = 0;
            let mut number_idx = 0;
            while nodes[node_idx] != Node::EndArray {
                let (value, i, j, k) = jsonc_list_to_serde_value(
                    &nodes[node_idx..],
                    &strings[string_idx..],
                    &numbers[number_idx..],
                );
                node_idx += i;
                string_idx += j;
                number_idx += k;
                arr.push(value);
            }
            (Value::Array(arr), node_idx + 1, string_idx, number_idx)
        }
        Node::StartObject => {
            let mut obj = serde_json::Map::new();
            let mut node_idx = 1;
            let mut string_idx = 0;
            let mut number_idx = 0;

            while nodes[node_idx] != Node::EndObject {
                let key = strings[string_idx].clone();
                string_idx += 1;
                node_idx += 1;
                let (value, i, j, k) = jsonc_list_to_serde_value(
                    &nodes[node_idx..],
                    &strings[string_idx..],
                    &numbers[number_idx..],
                );
                node_idx += i;
                string_idx += j;
                number_idx += k;
                obj.insert(key, value);
            }

            (Value::Object(obj), node_idx + 1, string_idx, number_idx)
        }
        _ => panic!("Invalid node value"),
    }
}

impl From<&Jsonc> for Value {
    fn from(jsonc: &Jsonc) -> Self {
        let (value, _, _, _) =
            jsonc_list_to_serde_value(&jsonc.nodes, &jsonc.strings, &jsonc.numbers);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jsonc_from_serde_json() {
        let value: Value = serde_json::from_str(r#"{"a":1,"b":[2,3],"c":{"d":4}}"#).unwrap();
        let jsonc = Jsonc::from(value);

        assert_eq!(
            jsonc,
            Jsonc {
                nodes: vec![
                    Node::StartObject,
                    Node::Key,
                    Node::Number,
                    Node::Key,
                    Node::StartArray,
                    Node::Number,
                    Node::Number,
                    Node::EndArray,
                    Node::Key,
                    Node::StartObject,
                    Node::Key,
                    Node::Number,
                    Node::EndObject,
                    Node::EndObject
                ],
                strings: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string()
                ],
                numbers: vec![1.0, 2.0, 3.0, 4.0]
            }
        );
    }

    #[test]
    fn test_jsonc_to_serde_json() {
        let value: Value =
            serde_json::from_str(r#"{"a":1.0,"b":[2.0,3.0],"c":{"d":4.0}}"#).unwrap();
        let jsonc = Jsonc {
            nodes: vec![
                Node::StartObject,
                Node::Key,
                Node::Number,
                Node::Key,
                Node::StartArray,
                Node::Number,
                Node::Number,
                Node::EndArray,
                Node::Key,
                Node::StartObject,
                Node::Key,
                Node::Number,
                Node::EndObject,
                Node::EndObject,
            ],
            strings: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ],
            numbers: vec![1.0, 2.0, 3.0, 4.0],
        };
        let value_from_jsonc = Value::from(&jsonc);

        assert_eq!(value_from_jsonc, value);
    }
}
