use crate::utils::parse_string;
use crate::value::{Json, Node};

pub fn parse_value(buf: &[u8]) -> Option<Json> {
    let mut json = Json::new();
    let mut parser = Parser::new(buf, &mut json);
    parser.parse();
    Some(json)
}

struct Parser<'a> {
    buf: &'a [u8],
    json: &'a mut Json,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(buf: &'a [u8], json: &'a mut Json) -> Parser<'a> {
        Self { buf, json, pos: 0 }
    }

    fn parse(&mut self) -> () {
        self.parse_value();
    }

    fn parse_value(&mut self) -> () {
        self.skip_whitespaces();
        match self.buf[self.pos] {
            b'n' => self.parse_null(),
            b't' => self.parse_true(),
            b'f' => self.parse_false(),
            b'"' => self.parse_string(),
            b'0'..=b'9' | b'-' => self.parse_number(),
            b'[' => self.parse_array(),
            b'{' => self.parse_object(),
            _ => panic!("unexpected character: {}", self.buf[self.pos] as char),
        }
    }

    fn parse_null(&mut self) -> () {
        self.must_match_by("null");
        self.json.offsets.push(None);
        self.json.nodes.push(Node::Null);
        self.step_by(4);
    }

    fn parse_true(&mut self) -> () {
        self.must_match_by("true");
        self.json.offsets.push(None);
        self.json.nodes.push(Node::True);
        self.step_by(4);
    }

    fn parse_false(&mut self) -> () {
        self.must_match_by("false");
        self.json.offsets.push(None);
        self.json.nodes.push(Node::False);
        self.step_by(5);
    }

    fn parse_string(&mut self) -> () {
        self.must_match(b'"');
        self.step();
        let start = self.pos;
        let mut has_escape = false;
        while self.pos < self.buf.len() {
            // escape character
            if self.buf[self.pos] == b'\\' {
                has_escape = true;
                self.step_by(2);
            } else if self.buf[self.pos] != b'"' {
                self.step();
            } else {
                break;
            }
        }
        let s = if has_escape {
            parse_string(&self.buf[start..self.pos]).unwrap()
        } else {
            std::str::from_utf8(&self.buf[start..self.pos])
                .unwrap()
                .to_string()
        };
        self.must_match(b'"');
        self.step();
        self.json.nodes.push(Node::String);
        self.json.offsets.push(Some(self.json.strings.len()));
        self.json.strings.push(s.to_string());
    }

    fn parse_number(&mut self) -> () {
        let start = self.pos;

        let positive = if self.buf[self.pos] == b'-' {
            self.step();
            false
        } else {
            true
        };

        let mut exp = false;
        let mut fraction = false;

        self.skip_digits();

        if self.peek(b'.') {
            fraction = true;
            self.step();
            self.skip_digits();
        } else if self.peek(b'e') || self.peek(b'E') {
            exp = true;
            self.step();
            if self.peek(b'-') || self.peek(b'+') {
                self.step();
            }
            self.skip_digits();
        }

        let s = std::str::from_utf8(&self.buf[start..self.pos]).unwrap();
        let n = s.parse::<f64>().unwrap();
        self.json.offsets.push(Some(self.json.numbers.len()));
        self.json.nodes.push(Node::Number);
        self.json.numbers.push(n);
    }

    fn parse_array(&mut self) -> () {
        self.must_match(b'[');
        self.json.offsets.push(None);
        self.json.nodes.push(Node::StartArray);
        self.step();
        loop {
            self.parse_value();
            self.skip_whitespaces();
            if self.peek(b',') {
                self.step();
            } else if self.peek(b']') {
                self.step();
                self.json.offsets.push(None);
                self.json.nodes.push(Node::EndArray);
                break;
            } else {
                panic!("unexpected character: {}", self.buf[self.pos] as char);
            }
        }
    }

    fn parse_object(&mut self) -> () {
        self.must_match(b'{');
        self.json.offsets.push(None);
        self.json.nodes.push(Node::StartObject);
        self.step();

        let mut first = true;

        loop {
            self.skip_whitespaces();
            if self.peek(b'}') {
                self.step();
                self.json.offsets.push(None);
                self.json.nodes.push(Node::EndObject);
                break;
            }

            if !first {
                self.must_match(b',');
                self.step();
            } else {
                first = false;
            }

            self.skip_whitespaces();
            self.json.offsets.push(Some(self.json.strings.len()));
            self.json.nodes.push(Node::Key);
            self.parse_string();
            // Pop the Node::String
            self.json.nodes.pop();
            self.json.offsets.pop();
            self.skip_whitespaces();
            self.must_match(b':');
            self.step();
            self.parse_value();
        }
    }

    fn skip_digits(&mut self) -> () {
        while self.pos < self.buf.len() {
            match self.buf[self.pos] {
                b'0'..=b'9' => self.step(),
                _ => break,
            }
        }
    }

    fn skip_whitespaces(&mut self) {
        while self.pos < self.buf.len() {
            if self.buf[self.pos].is_ascii_whitespace() {
                self.step();
            } else {
                break;
            }
            // jsonb by databend also skips the "\\n|\\r|\\t" characters, don't know why
        }
    }

    fn step(&mut self) {
        if self.pos < self.buf.len() {
            self.pos += 1;
        } else {
            panic!("unexpected end of input");
        }
    }

    fn step_by(&mut self, n: usize) {
        if self.pos + n <= self.buf.len() {
            self.pos += n;
        } else {
            panic!("unexpected end of input");
        }
    }

    fn peek(&mut self, c: u8) -> bool {
        if self.pos < self.buf.len() && self.buf[self.pos] == c {
            true
        } else {
            false
        }
    }

    fn peek_by(&mut self, s: &str) -> bool {
        let s = s.as_bytes();
        if self.pos + s.len() <= self.buf.len() && &self.buf[self.pos..self.pos + s.len()] == s {
            true
        } else {
            false
        }
    }

    fn must_match(&mut self, c: u8) -> bool {
        if self.peek(c) {
            true
        } else {
            panic!("unexpected character: {}", c as char);
        }
    }

    fn must_match_by(&mut self, s: &str) -> bool {
        if self.peek_by(s) {
            true
        } else {
            panic!("unexpected string: {}", s);
        }
    }
}
