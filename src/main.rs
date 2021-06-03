use serde_json::{Result, Value};

fn main() { 
    //println!("{}", test("Hello"))
}

pub fn json_decode(string:&str) -> Result<Value> {
    let result: Value = serde_json::from_str(string)?;
    Ok(result)
}

pub fn test(string:&str) -> &str{
    return string;
}