use std::sync::Arc;

use arrow::{array::{ArrayRef, Float64Array, StringArray, StructArray, UInt64Array, UInt8Array}, datatypes::{DataType, Field, Fields}};
use datafusion_common::ScalarValue;

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

impl Node {
    fn to_u8(&self) -> u8 {
        match self {
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

    pub fn try_to_scalar_value(&self) -> Option<ScalarValue> {
        let nodes = self.nodes.iter().map(|n| n.to_u8()).collect::<Vec<u8>>();
        let offsets = self
            .offsets
            .iter()
            .map(|o| match o {
                Some(o) => *o as u64,
                None => 0,
            })
            .collect::<Vec<u64>>();
        let strings = self.strings.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        let numbers = self.numbers.iter().map(|n| *n).collect::<Vec<f64>>();

        let nodes = Arc::new(UInt8Array::from(nodes));
        let offsets = Arc::new(UInt64Array::from(offsets));
        let strings = Arc::new(StringArray::from(strings));
        let numbers = Arc::new(Float64Array::from(numbers));

        let values: Vec<ArrayRef> = vec![nodes, offsets, strings, numbers];

        let fields = Fields::from(vec![
            Field::new("nodes", DataType::UInt8, false),
            Field::new("offsets", DataType::UInt64, false),
            Field::new("strings", DataType::Utf8, false),
            Field::new("numbers", DataType::Float64, false),
        ]);
        let nulls = None;
        let arr = StructArray::new(fields, values, nulls);

        Some(ScalarValue::Struct(Arc::new(arr)))
    }
}
