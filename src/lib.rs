use std::{collections::HashMap, error::Error, fmt};

#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub enum JsonToken {
    OpenObject,
    //
    ////
    CloseObject,
    DoubleQuote,
    Number(i32),
    JsonString(String),
    Colon,
    Comma,
    Space,
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
}

impl JsonParser {
    pub fn new(tk: Tokenizer) -> Self {
        Self { tokenizer: tk }
    }

    pub fn parse(&mut self) -> Result<Content, ParseError> {
        let t = self.tokenizer.get_token()?;
        if let JsonToken::OpenObject = t {
            let mut content = Content::Object(HashMap::new());
            self.parse_object(&mut content)?;
            // self.root = content;
            println!("parse result: {:?}", content);
            Ok(content)
        } else {
            println!("only object");
            Err(ParseError {
                details: "jj".into(),
            }
            .into())
        }
    }
    pub fn parse_object(&mut self, content: &mut Content) -> Result<(), ParseError> {
        while let Ok(r) = self.tokenizer.get_token() {
            if let JsonToken::CloseObject = r {
                break;
            }
            let key: String;
            if let JsonToken::JsonString(r) = r {
                key = r.to_string();
            } else {
                return Err(ParseError::new("must be string"));
            }

            if let JsonToken::Colon = self.tokenizer.get_token()? {
            } else {
                return Err(ParseError::new("must be :"));
            }

            let mut v: Content;
            match self.tokenizer.get_token()? {
                JsonToken::JsonString(r) => {
                    v = Content::StringLiteral(r.clone());
                    if let Content::Object(o) = content {
                        o.insert(key.to_string(), v);
                    }
                }
                JsonToken::Number(r) => {
                    v = Content::Number(r);
                    if let Content::Object(o) = content {
                        o.insert(key.to_string(), v);
                    }
                }
                JsonToken::OpenObject => {
                    v = Content::Object(HashMap::new());
                    self.parse_object(&mut v)?;
                    if let Content::Object(o) = content {
                        o.insert(key.to_string(), v);
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
        // <string><colon> <string>|<number>
        // [<comma><string><colon> <string>|<number>]*
        Ok(())
    }
    pub fn parse_array(&mut self) -> Result<(), ()> {
        // no impl
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
    pub fn get_token(&mut self) -> Result<JsonToken, ParseError> {
        // let mut pos = 0;

        loop {
            let c = self
                .source
                .chars()
                .nth(self.pos)
                .ok_or(ParseError::new("index invalid"))?;
            if c.is_numeric() {
                let mut t = String::new();
                while self
                    .source
                    .chars()
                    .nth(self.pos)
                    .ok_or(ParseError::new("index invalid"))?
                    .is_numeric()
                {
                    let a = (self.source.chars().nth(self.pos))
                        .ok_or(ParseError::new("index invalid"))?;
                    t += &a.to_string();
                    self.pos += 1
                }
                let tnum = t.parse().or(Err(ParseError::new("string to num".into())))?;
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
                        .ok_or(ParseError::new("index error"))?;
                    if c == '\\' {
                        self.pos += 1;
                        let c = self
                            .source
                            .chars()
                            .nth(self.pos)
                            .ok_or(ParseError::new("index error"))?;
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
                return Err(ParseError::new("failed to parse"));
            }
        }
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
