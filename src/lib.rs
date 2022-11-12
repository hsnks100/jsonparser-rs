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
pub enum Token<R> {
    Start { rule: R, pos: i32 },
}

#[derive(Debug)]
pub struct JsonTokenWrap {
    pos: usize,
    kind: JsonToken,
}
#[derive(Debug)]
pub enum JsonToken {
    OpenObject,
    CloseObject,
    OpenArray,
    CloseArray,
    DoubleQuote,
    Number(i32),
    JsonString(String),
    Colon,
    Comma,
    Space,
}

#[derive(Debug)]
pub enum Content {
    Number(i32),
    StringLiteral(String),
    Object(HashMap<String, Content>),
    Array(Vec<Content>),
}

impl Content {
    pub fn get(&mut self, key: String) -> Option<&Content> {
        if let Content::Object(h) = self {
            h.get(&key)
        } else {
            None
        }
    }
    pub fn at(&mut self, index: usize) -> Option<&Content> {
        if let Content::Array(h) = self {
            h.get(index)
        } else {
            None
        }
    }
}

pub struct JsonParser {
    tokenizer: Tokenizer,
}

impl JsonParser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer } // <--
    }

    pub fn parse(&mut self) -> Result<Content, ParseError> {
        let t = self.tokenizer.get_token()?;
        match t.kind {
            JsonToken::OpenObject => {
                let mut content = Content::Object(HashMap::new());
                self.parse_object(&mut content)?;
                // self.root = content;
                println!("parse result: {:?}", content);
                Ok(content)
            }
            JsonToken::OpenArray => {
                let mut content = Content::Array(Vec::new());
                self.parse_array(&mut content)?;
                println!("parse result: {:?}", content);
                Ok(content)
            }
            _ => {
                println!("parse error");
                Err(ParseError::new("parse err"))
            }
        }
    }
    pub fn parse_object(&mut self, content: &mut Content) -> Result<(), ParseError> {
        while let Ok(r) = self.tokenizer.get_token() {
            let key = match r.kind {
                JsonToken::CloseObject => {
                    break;
                }
                JsonToken::JsonString(value) => value.clone(),
                _ => {
                    // return Err(ParseError::new("must be string"));
                    return Err(ParseError::new(&format!("must be string: pos: {}", r.pos)));
                }
            };
            match self.tokenizer.get_token()?.kind {
                JsonToken::Colon => {}
                _ => {
                    return Err(ParseError::new("must be :"));
                }
            }
            match (self.tokenizer.get_token()?.kind, &mut *content) {
                (JsonToken::JsonString(value), Content::Object(o)) => {
                    let v = Content::StringLiteral(value.clone());
                    o.insert(key.to_string(), v);
                }
                (JsonToken::Number(value), Content::Object(o)) => {
                    let v = Content::Number(value);
                    o.insert(key.to_string(), v);
                }
                (JsonToken::OpenObject, Content::Object(o)) => {
                    let mut v = Content::Object(HashMap::new());
                    self.parse_object(&mut v)?;
                    o.insert(key.to_string(), v);
                }
                (JsonToken::OpenArray, Content::Object(o)) => {
                    // let mut v = Content::Object(HashMap::new());
                    // self.parse_object(&mut v)?;
                    // o.insert(key.to_string(), v);
                    let mut content = Content::Array(Vec::new());
                    self.parse_array(&mut content)?;
                    o.insert(key.to_string(), content);
                }
                _ => {}
            }
            match self.tokenizer.get_token()?.kind {
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
    pub fn parse_array(&mut self, content: &mut Content) -> Result<(), ParseError> {
        // no impl
        // []
        // [a]
        // [a,b,c]
        while let Ok(r) = self.tokenizer.get_token() {
            if let JsonToken::CloseArray = r.kind {
                break;
            };
            match (r.kind, &mut *content) {
                (JsonToken::JsonString(value), Content::Array(o)) => {
                    let v = Content::StringLiteral(value.clone());
                    o.push(v)
                }
                (JsonToken::Number(value), Content::Array(o)) => {
                    let v = Content::Number(value);
                    o.push(v);
                }
                (JsonToken::OpenObject, Content::Array(o)) => {
                    let mut v = Content::Object(HashMap::new());
                    self.parse_object(&mut v)?;
                    o.push(v);
                }
                _ => {}
            }
            match self.tokenizer.get_token()?.kind {
                JsonToken::Comma => {
                    // ignore
                }
                JsonToken::CloseArray => {
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
pub struct Tokenizer {
    source: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
        }
    }
    pub fn get_token(&mut self) -> Result<JsonTokenWrap, ParseError> {
        loop {
            let c = *self.source.get(self.pos).ok_or(ParseError::new("end"))?;
            if c.is_numeric() {
                let mut t = String::new();
                let b = self.pos;
                while self
                    .source
                    .get(self.pos)
                    .ok_or(ParseError::new("end"))?
                    .is_numeric()
                {
                    let a = self.source[self.pos];
                    t += &a.to_string();
                    self.pos += 1
                }
                let tnum = t.parse().or(Err(ParseError::new("string to num".into())))?;
                let r = JsonToken::Number(tnum);
                return Ok(JsonTokenWrap { pos: b, kind: r });
            } else if c == '{' {
                self.pos += 1;
                return Ok(JsonTokenWrap {
                    pos: self.pos - 1,
                    kind: JsonToken::OpenObject,
                });
            } else if c == '}' {
                self.pos += 1;
                return Ok(JsonTokenWrap {
                    pos: self.pos - 1,
                    kind: JsonToken::CloseObject,
                });
            } else if c == '"' {
                let mut t = String::new();
                let b = self.pos;
                loop {
                    self.pos += 1;
                    let c = self.source[self.pos];
                    if c == '\\' {
                        self.pos += 1;
                        let c = self.source[self.pos];
                        t += &c.to_string();
                        continue;
                    }
                    if c == '"' {
                        break;
                    }
                    t += &c.to_string();
                }
                self.pos += 1;
                return Ok(JsonTokenWrap {
                    pos: b,
                    kind: JsonToken::JsonString(t),
                });
            } else if c == ',' {
                self.pos += 1;
                return Ok(JsonTokenWrap {
                    pos: self.pos - 1,
                    kind: JsonToken::Comma,
                });
            } else if c == ' ' {
                self.pos += 1;
            } else if c == ':' {
                self.pos += 1;
                return Ok(JsonTokenWrap {
                    pos: self.pos - 1,
                    kind: JsonToken::Colon,
                });
            } else if c == '[' {
                self.pos += 1;
                return Ok(JsonTokenWrap {
                    pos: self.pos - 1,
                    kind: JsonToken::OpenArray,
                });
            } else if c == ']' {
                self.pos += 1;
                return Ok(JsonTokenWrap {
                    pos: self.pos - 1,
                    kind: JsonToken::CloseArray,
                });
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
