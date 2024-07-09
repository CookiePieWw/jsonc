use jsonc::decoder::decode;
use jsonc::parser::parse_value;
use jsonc::value::{Jsonc, Node};

fn test_parser() {
    let json = r#"
        {
            "name": "John Doe",
            "age": 43,
            "is_student": false,
            "scores": [100, 98, 100]
        }
    "#;
    let json = json.as_bytes();
    let parsed_json = parse_value(json);

    let mut expected_json = Jsonc::new();
    let nodes = vec![
        Node::StartObject,
        Node::Key,
        Node::String,
        Node::Key,
        Node::Number,
        Node::Key,
        Node::False,
        Node::Key,
        Node::StartArray,
        Node::Number,
        Node::Number,
        Node::Number,
        Node::EndArray,
        Node::EndObject,
    ];
    let strings = vec![
        "name".to_string(),
        "John Doe".to_string(),
        "age".to_string(),
        "is_student".to_string(),
        "scores".to_string(),
    ];
    let numbers = vec![43.0, 100.0, 98.0, 100.0];

    expected_json.nodes = nodes;
    expected_json.strings = strings;
    expected_json.numbers = numbers;

    assert_eq!(parsed_json, expected_json);
}

fn test_decoder() {
    let json = r#"
        {
            "name": "John Doe",
            "age": 43,
            "is_student": false,
            "scores": [100, 98, 100]
        }
    "#;
    let json = json.as_bytes();
    let parsed_json = parse_value(json);
    let decoded_json = decode(&parsed_json);

    let expected_json = r#"{"name":"John Doe","age":43,"is_student":false,"scores":[100,98,100]}"#;

    assert_eq!(decoded_json, expected_json);
}

#[test]
fn test() {
    test_parser();
    test_decoder();
}
