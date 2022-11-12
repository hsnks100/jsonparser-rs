use serde_json::Value;
use std::{collections::HashMap, error::Error};

use libjsonparser::{Content, JsonParser, ParseError, Tokenizer};

fn parse_test(json_string: &str) -> Result<Content, Box<dyn Error>> {
    let mut jp = JsonParser::new2(json_string);
    let content = jp.parse()?;
    Ok(content)
}
fn main() -> Result<(), ParseError> {
    let json_string =
        r#"{"vvv":1234, "array":[101,102,103,104, "hi", {"name":"ksoo", "age":25}]}}"#;
    let c = JsonParser::content_from_str(json_string)?;
    println!("vvv: {:?}", c.get("vvv"));
    println!("array[0]: {:?}", c.get("array").at(0));
    println!("array[1]: {:?}", c.get("array").at(1));
    println!("array[2]: {:?}", c.get("array").at(2));
    println!("array[3]: {:?}", c.get("array").at(3));
    println!("array[4]: {:?}", c.get("array").at(4));
    println!("array[4].name: {:?}", c.get("array").at(5).get("name"));
    println!("array[4].age: {:?}", c.get("array").at(5).get("age"));
    Ok(())
}
