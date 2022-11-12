use std::{collections::HashMap, error::Error};

use libjsonparser::{Content, JsonParser, Tokenizer};

fn parse_test(json_string: &str) -> Result<Content, Box<dyn Error>> {
    println!("json string: {}", json_string);
    {
        let mut t = Tokenizer::new(json_string.to_string());
        loop {
            let r = t.get_token();
            match r {
                Ok(e) => {}
                Err(e) => {
                    break;
                }
            }
        }
    }

    let mut t = Tokenizer::new(json_string.to_string());
    let mut jp = JsonParser::new(t);
    let content = jp.parse()?;
    Ok(content)
}
fn main() -> Result<(), Box<dyn Error>> {
    // let jsonString = r#"{"vvv" : 3434,     "field":"valu\"e1"}"#;
    let json_string = r#"{"vvv" : "한글"}444444"#;
    parse_test(json_string)?;
    let json_string = r#"{"vvv" : "한글"}"#;
    parse_test(json_string)?;
    let json_string = r#"{"vvv" : 3434,     "field":"valu\"e1"}"#;
    parse_test(json_string)?;
    let json_string = r#"{}"#;
    parse_test(json_string)?;
    let json_string = r#"{"vvv":1234, "obj":{"depth1":1, "depth2":2}}"#;
    parse_test(json_string)?;
    let json_string = r#"{"vvv":1234, "obj":{}}"#;
    parse_test(json_string)?;
    let json_string = r#"[1,2,3,4, "str", {"vvv":1234, "obj":{}}]"#;
    parse_test(json_string)?;
    let json_string = r#"{"vvv":1234, "array":[1,2,3,4, "hi"]}}"#;
    let mut c = parse_test(json_string)?;
    let d = c.get("vvv".to_string());
    match d {
        Some(s) => match s {
            Content::Number(v) => {
                println!("number: {}", v);
            }
            _ => {}
        },
        None => {}
    }
    if d.is_some() {}
    Ok(())
}
