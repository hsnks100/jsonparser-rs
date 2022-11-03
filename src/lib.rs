use std::{collections::HashMap, error::Error, fmt, string::ParseError};

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub enum JsonToken {
    OpenObject,
    CloseObject,
    DoubleQuote,
    Number(i32),
    JsonString(String),
    Colon,
    Comma,
    Space,
}

pub enum Operator {
    Plus,
    Minus,
}

// {"abcd":1234, "fddfg":{"name":"ksoo"}}

// vector<int> graph[100];
// graph[0]: root
//
impl JsonToken {}

#[derive(Debug)]
pub enum Content {
    Number(i32),
    StringLiteral(String),
    Object(HashMap<String, Content>),
    Array(Vec<Content>),
}

pub struct JsonParser {
    tokenizer: Tokenizer,
    root: Option<Content>,
}

impl JsonParser {
    pub fn new(tk: Tokenizer) -> Self {
        Self {
            tokenizer: tk,
            root: None,
        }
    }

    pub fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        let t = self.tokenizer.get_token()?;
        if let JsonToken::OpenObject = t {
            let mut content = Some(Content::Object(HashMap::new()));
            let c = content.as_mut().ok_or_else(|| MyError::new(""))?;
            self.parse_object(c)?;
            self.root = content;
            println!("parse result: {:?}", self.root.as_ref().ok_or("")?);
        } else {
            println!("only object");
            return Err("".into());
        }
        Ok(())
    }
    pub fn parse_object(&mut self, content: &mut Content) -> Result<(), Box<dyn Error>> {
        while let Ok(r) = self.tokenizer.get_token() {
            match r {
                JsonToken::CloseObject => {
                    break;
                }
                _ => {
                    let key: String;
                    if let JsonToken::JsonString(r) = r {
                        key = r.clone();
                    } else {
                        return Err("".into());
                    }
                    if let JsonToken::Colon = self.tokenizer.get_token()? {
                    } else {
                        return Err("".into());
                    }

                    let mut v: Content;
                    match self.tokenizer.get_token()? {
                        JsonToken::JsonString(r) => {
                            v = Content::StringLiteral(r.clone());
                            if let Content::Object(o) = content {
                                o.insert(key, v);
                            }
                        }
                        JsonToken::Number(r) => {
                            v = Content::Number(r);
                            if let Content::Object(o) = content {
                                o.insert(key, v);
                            }
                        }
                        JsonToken::OpenObject => {
                            v = Content::Object(HashMap::new());
                            self.parse_object(&mut v)?;
                            if let Content::Object(o) = content {
                                o.insert(key, v);
                            }
                        }
                        _ => {}
                    }
                    match self.tokenizer.get_token()? {
                        JsonToken::Comma => {
                            // ignore
                        }
                        JsonToken::CloseObject => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
        // <string><colon> <string>|<number>
        // [<comma><string><colon> <string>|<number>]*
        Ok(())
    }
    pub fn parse_array(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
pub struct Tokenizer {
    source: String,
    pos: usize,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self { source, pos: 0 }
    }
    pub fn get_token(&mut self) -> Result<JsonToken, Box<dyn Error>> {
        // let mut pos = 0;
        while let c = self
            .source
            .chars()
            .nth(self.pos)
            .ok_or_else(|| MyError::new("index invalid"))?
        {
            if c.is_numeric() {
                let mut t = String::new();
                while self
                    .source
                    .chars()
                    .nth(self.pos)
                    .ok_or(MyError::new("index invalid"))?
                    .is_numeric()
                {
                    let a =
                        (self.source.chars().nth(self.pos)).ok_or(MyError::new("index invalid"))?;
                    t += &a.to_string();
                    self.pos += 1
                }
                let tnum = t.parse().or(Err("string to num"))?;
                let r = JsonToken::Number(tnum);
                return Ok(r);
            } else if c == '{' {
                self.pos += 1;
                return Ok(JsonToken::OpenObject);
            } else if c == '}' {
                self.pos += 1;
                return Ok(JsonToken::CloseObject);
            } else if c == '"' {
                let mut t = String::new();
                loop {
                    self.pos += 1;
                    let c = self
                        .source
                        .chars()
                        .nth(self.pos)
                        .ok_or(MyError::new("index error"))?;
                    if c == '\\' {
                        self.pos += 1;
                        let c = self
                            .source
                            .chars()
                            .nth(self.pos)
                            .ok_or(MyError::new("index error"))?;
                        t += &c.to_string();
                        continue;
                    }
                    if c == '"' {
                        break;
                    }
                    t += &c.to_string();
                }
                self.pos += 1;
                return Ok(JsonToken::JsonString(t));
            } else if c == ',' {
                self.pos += 1;
                return Ok(JsonToken::Comma);
            } else if c == ' ' {
                self.pos += 1;
            } else if c == ':' {
                self.pos += 1;
                return Ok(JsonToken::Colon);
            } else {
                return Err("failed to parse".into());
            }
        }
        Err("failed to parse".into())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), ()> {
        Ok(())
    }
}
