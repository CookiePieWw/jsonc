use std::sync::Arc;

use arrow::datatypes::{DataType, Field, Fields};

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

#[derive(Debug, PartialEq)]
pub struct Json {
    pub nodes: Vec<Node>,
    pub offsets: Vec<Option<usize>>,
    pub strings: Vec<String>,
    pub numbers: Vec<f64>,
}

impl Json {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            offsets: Vec::new(),
            strings: Vec::new(),
            numbers: Vec::new(),
        }
    }

    fn name(&self) -> String {
        "Json".to_string()
    }

    fn as_arrow_type(&self) -> DataType {
        DataType::Struct(Fields::from(vec![
            Field::new(
                "nodes",
                DataType::List(Arc::new(Field::new("item", DataType::UInt8, true))),
                false,
            ),
            Field::new(
                "offsets",
                DataType::List(Arc::new(Field::new("item", DataType::UInt64, true))),
                false,
            ),
            Field::new(
                "strings",
                DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
                false,
            ),
            Field::new(
                "numbers",
                DataType::List(Arc::new(Field::new("item", DataType::Float64, true))),
                false,
            ),
        ]))
    }
}
