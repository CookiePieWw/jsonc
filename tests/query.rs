use jsonc::parser::parse_value;

fn test_json_get_1() {
    let json_str = std::fs::read_to_string("data/twitter.json").unwrap();
    let parsed_json = parse_value(json_str.as_bytes());
    let result = parsed_json.get(&["\"search_metadata\"", "\"max_id_str\""]).unwrap();
    assert_eq!(&result, "\"505874924095815681\"");
}

fn test_json_get_2() {
    let json_str = std::fs::read_to_string("data/canada.json").unwrap();
    let parsed_json = parse_value(json_str.as_bytes());
    let result = parsed_json.get(&["\"type\""]).unwrap();
    assert_eq!(&result, "\"FeatureCollection\"");
}

fn test_json_get_3() {
    let json_str = std::fs::read_to_string("data/citm_catalog.json").unwrap();
    let parsed_json = parse_value(json_str.as_bytes());
    let result = parsed_json.get(&["\"areaNames\"", "\"205705994\""]).unwrap();
    assert_eq!(&result, "\"1er balcon central\"");
    let result = parsed_json.get(&["\"topicNames\"", "\"324846100\""]).unwrap();
    assert_eq!(&result, "\"Formations musicales\"");
}

#[test]
fn test() {
    test_json_get_1();
    test_json_get_2();
    test_json_get_3()
}
