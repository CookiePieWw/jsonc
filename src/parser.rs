use core::panic;

use jsonb::util::parse_string;

use crate::value::{Jsonc, Node};

pub fn parse_value(buf: &[u8]) -> Jsonc {
    let mut json = Jsonc::default();
    let mut parser = Parser::new(buf, &mut json);
    parser.parse();
    json
}

struct Parser<'a> {
    buf: &'a [u8],
    json: &'a mut Jsonc,
    idx: usize,
}

impl<'a> Parser<'a> {
    fn new(buf: &'a [u8], json: &'a mut Jsonc) -> Parser<'a> {
        Self { buf, json, idx: 0 }
    }

    fn parse(&mut self) {
        self.parse_json_value();
        self.skip_unused();
        if self.idx < self.buf.len() {
            self.step();
            panic!("Unexpected trailing characters")
        }
    }

    fn parse_json_value(&mut self) {
        self.skip_unused();
        let c = self.next();
        match c {
            b'n' => self.parse_json_null(),
            b't' => self.parse_json_true(),
            b'f' => self.parse_json_false(),
            b'0'..=b'9' | b'-' => self.parse_json_number(),
            b'"' => self.parse_json_string(),
            b'[' => self.parse_json_array(),
            b'{' => self.parse_json_object(),
            _ => {
                self.step();
                panic!("Unexpected character")
            }
        };
    }

    fn next(&mut self) -> &u8 {
        match self.buf.get(self.idx) {
            Some(c) => c,
            None => panic!("Unexpected EOF"),
        }
    }

    fn must_is(&mut self, c: u8) {
        match self.buf.get(self.idx) {
            Some(v) => {
                self.step();
                if v != &c {
                    panic!("Unexpected character")
                }
            }
            None => panic!("Unexpected EOF"),
        }
    }

    fn check_next(&mut self, c: u8) -> bool {
        if self.idx < self.buf.len() {
            let v = self.buf.get(self.idx).unwrap();
            if v == &c {
                return true;
            }
        }
        false
    }

    fn check_next_either(&mut self, c1: u8, c2: u8) -> bool {
        if self.idx < self.buf.len() {
            let v = self.buf.get(self.idx).unwrap();
            if v == &c1 || v == &c2 {
                return true;
            }
        }
        false
    }

    fn check_digit(&mut self) -> bool {
        if self.idx < self.buf.len() {
            let v = self.buf.get(self.idx).unwrap();
            if v.is_ascii_digit() {
                return true;
            }
        }
        false
    }

    fn step_digits(&mut self) -> usize {
        if self.idx == self.buf.len() {
            panic!("Unexpected EOF")
        }
        let mut len = 0;
        while self.idx < self.buf.len() {
            let c = self.buf.get(self.idx).unwrap();
            if !c.is_ascii_digit() {
                break;
            }
            len += 1;
            self.step();
        }
        len
    }

    #[inline]
    fn step(&mut self) {
        self.idx += 1;
    }

    #[inline]
    fn step_by(&mut self, n: usize) {
        self.idx += n;
    }

    #[inline]
    fn skip_unused(&mut self) {
        while self.idx < self.buf.len() {
            let c = self.buf.get(self.idx).unwrap();
            if c.is_ascii_whitespace() {
                self.step();
                continue;
            }
            // Allow parse escaped white space
            if *c == b'\\' {
                if self.idx + 1 < self.buf.len()
                    && matches!(self.buf[self.idx + 1], b'n' | b'r' | b't')
                {
                    self.step_by(2);
                    continue;
                }
                if self.idx + 3 < self.buf.len()
                    && self.buf[self.idx + 1] == b'x'
                    && self.buf[self.idx + 2] == b'0'
                    && self.buf[self.idx + 3] == b'C'
                {
                    self.step_by(4);
                    continue;
                }
            }
            break;
        }
    }

    fn parse_json_null(&mut self) {
        let data = [b'n', b'u', b'l', b'l'];
        for v in data.into_iter() {
            self.must_is(v);
        }
        self.json.nodes.push(Node::Null);
    }

    fn parse_json_true(&mut self) {
        let data = [b't', b'r', b'u', b'e'];
        for v in data.into_iter() {
            self.must_is(v);
        }
        self.json.nodes.push(Node::True);
    }

    fn parse_json_false(&mut self) {
        let data = [b'f', b'a', b'l', b's', b'e'];
        for v in data.into_iter() {
            self.must_is(v);
        }
        self.json.nodes.push(Node::False);
    }

    fn parse_json_number(&mut self) {
        let start_idx = self.idx;

        let mut has_fraction = false;
        let mut has_exponent = false;
        let mut negative: bool = false;

        if self.check_next(b'-') {
            negative = true;
            self.step();
        }
        if self.check_next(b'0') {
            self.step();
            if self.check_digit() {
                self.step();
                panic!("Invalid number value")
            }
        } else {
            let len = self.step_digits();
            if len == 0 {
                self.step();
                panic!("Invalid number value")
            }
        }
        if self.check_next(b'.') {
            has_fraction = true;
            self.step();
            let len = self.step_digits();
            if len == 0 {
                self.step();
                panic!("Invalid number value")
            }
        }
        if self.check_next_either(b'E', b'e') {
            has_exponent = true;
            self.step();
            if self.check_next_either(b'+', b'-') {
                self.step();
            }
            let len = self.step_digits();
            if len == 0 {
                self.step();
                panic!("Invalid number value")
            }
        }
        let s = unsafe { std::str::from_utf8_unchecked(&self.buf[start_idx..self.idx]) };

        if !has_fraction && !has_exponent {
            if !negative {
                if let Ok(v) = s.parse::<u64>() {
                    self.json.nodes.push(Node::Number);
                    self.json.numbers.push(v as f64);
                    return;
                }
            } else if let Ok(v) = s.parse::<i64>() {
                self.json.nodes.push(Node::Number);
                self.json.numbers.push(v as f64);
                return;
            }
        }

        match fast_float::parse(s) {
            Ok(v) => {
                self.json.nodes.push(Node::Number);
                self.json.numbers.push(v);
            }
            Err(_) => panic!("Invalid number value"),
        }
    }

    fn parse_json_string(&mut self) {
        self.must_is(b'"');

        let start_idx = self.idx;
        let mut escapes = 0;
        loop {
            let c = self.next();
            match c {
                b'\\' => {
                    self.step();
                    escapes += 1;
                    let next_c = self.next();
                    if *next_c == b'u' {
                        self.step();
                        let next_c = self.next();
                        if *next_c == b'{' {
                            self.step_by(6);
                        } else {
                            self.step_by(4);
                        }
                    } else {
                        self.step();
                    }
                    continue;
                }
                b'"' => {
                    self.step();
                    break;
                }
                _ => {}
            }
            self.step();
        }

        let data = &self.buf[start_idx..self.idx - 1];
        let val = if escapes > 0 {
            let len = self.idx - 1 - start_idx - escapes;
            let mut idx = start_idx + 1;
            parse_string(data, len, &mut idx).unwrap()
        } else {
            std::str::from_utf8(data).unwrap().to_string()
        };
        self.json.nodes.push(Node::String);
        self.json.strings.push(val);
    }

    fn parse_json_array(&mut self) {
        self.must_is(b'[');

        self.json.nodes.push(Node::StartArray);
        let mut first = true;
        loop {
            self.skip_unused();
            let c = self.next();
            if *c == b']' {
                self.step();
                break;
            }
            if !first {
                if *c != b',' {
                    panic!("Unexpected character")
                }
                self.step();
            }
            first = false;
            self.parse_json_value();
        }
        self.json.nodes.push(Node::EndArray);
    }

    fn parse_json_object(&mut self) {
        self.must_is(b'{');

        let mut first = true;
        self.json.nodes.push(Node::StartObject);
        loop {
            self.skip_unused();
            let c = self.next();
            if *c == b'}' {
                self.step();
                break;
            }
            if !first {
                if *c != b',' {
                    panic!("Unexpected character")
                }
                self.step();
            }
            first = false;
            self.parse_json_value();
            if !matches!(self.json.nodes.pop(), Some(Node::String)) {
                panic!("Expected string key")
            }
            self.json.nodes.push(Node::Key);
            self.skip_unused();
            let c = self.next();
            if *c != b':' {
                panic!("Unexpected character")
            }
            self.step();
            self.parse_json_value();
        }
        self.json.nodes.push(Node::EndObject);
    }
}
