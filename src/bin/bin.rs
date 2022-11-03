use std::{collections::HashMap, error::Error};

use libjsonparser::{Content, JsonParser, Tokenizer};

fn parse_test(json_string: &str) -> Result<(), Box<dyn Error>> {
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
    jp.parse()?;
    println!("--------------------------");
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    // let jsonString = r#"{"vvv" : 3434,     "field":"valu\"e1"}"#;
    let json_string = r#"{"vvv" : 3434}"#;
    parse_test(json_string)?;
    let json_string = r#"{"vvv" : 3434,     "field":"valu\"e1"}"#;
    parse_test(json_string)?;
    let json_string = r#"{}"#;
    parse_test(json_string)?;
    let json_string = r#"{"vvv":1234, "obj":{"depth1":1, "depth2":2}}"#;
    parse_test(json_string)?;
    let json_string = r#"{"vvv":1234, "obj":{}}"#;
    parse_test(json_string)?;
    Ok(())
}
