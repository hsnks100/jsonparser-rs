use std::{collections::HashMap, error::Error, fmt};

use anyhow::{self, Context};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{}, pos: {}", .message, .pos)]
    InvalidInput { message: String, pos: usize },
    #[error("token error")]
    TokenError,
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
    Null,
    Number(i32),
    StringLiteral(String),
    Object(HashMap<String, Content>),
    Array(Vec<Content>),
}

impl Content {
    pub fn get(&self, key: &str) -> &Content {
        if let Content::Object(h) = self {
            let r = h.get(key);
            match r {
                Some(r) => r,
                None => &Content::Null,
            }
        } else {
            &Content::Null
        }
    }
    pub fn at(&self, index: usize) -> &Content {
        if let Content::Array(h) = self {
            let r = h.get(index);
            match r {
                Some(r) => r,
                None => &Content::Null,
            }
        } else {
            &Content::Null
        }
    }
}

pub struct JsonParser {
    tokenizer: Tokenizer,
}

impl JsonParser {
    pub fn content_from_str(json_str: &str) -> Result<Content, ParseError> {
        let mut jp = JsonParser::new2(json_str);
        let content = jp.parse()?;
        Ok(content)
    }
    pub fn new2(json_str: &str) -> Self {
        Self {
            tokenizer: Tokenizer::new(json_str.to_string()),
        }
    }
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
            _ => Err(ParseError::InvalidInput {
                message: "parse error".to_string(),
                pos: t.pos,
            }),
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
                    return Err(ParseError::InvalidInput {
                        message: "must be string".to_string(),
                        pos: r.pos,
                    });
                    // return Err(ParseError::new("must be string"));
                    // return Err(ParseError::InvalidInput(message: "must be string".to_string(), pos: {}", r.pos));
                }
            };
            match self.tokenizer.get_token()?.kind {
                JsonToken::Colon => {}
                _ => {
                    return Err(ParseError::InvalidInput {
                        message: "must be :".to_string(),
                        pos: r.pos,
                    })
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
            let c = *self.source.get(self.pos).ok_or(ParseError::InvalidInput {
                message: "EOF".to_string(),
                pos: self.pos,
            })?;
            if c.is_numeric() {
                let mut t = String::new();
                let b = self.pos;
                while self
                    .source
                    .get(self.pos)
                    .ok_or(ParseError::InvalidInput {
                        message: "end".to_string(),
                        pos: self.pos,
                    })?
                    .is_numeric()
                {
                    let a = self.source[self.pos];
                    t += &a.to_string();
                    self.pos += 1
                }
                let tnum = t.parse().or(Err(ParseError::InvalidInput {
                    message: "string to num".to_string(),
                    pos: b,
                }))?;
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
                return Err(ParseError::InvalidInput {
                    message: "failed to parse".to_string(),
                    pos: self.pos,
                });
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
