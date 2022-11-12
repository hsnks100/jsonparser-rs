# json parse 연습

# example

```bash
cargo run
```

```rust
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
-------------------------------
output
rse result: Object({"array": Array([Number(101), Number(102), Number(103), Number(104), StringLiteral("hi"), Object({"name": StringLiteral("ksoo"), "age": Number(25)})]), "vvv": Number(1234)})
vvv: Number(1234)
array[0]: Number(101)
array[1]: Number(102)
array[2]: Number(103)
array[3]: Number(104)
array[4]: StringLiteral("hi")
array[4].name: StringLiteral("ksoo")
array[4].age: Number(25)
```
