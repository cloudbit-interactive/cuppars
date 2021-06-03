use serde_json::{Result, Value};

fn main() { }

pub fn json_decode(string:&str) -> Result<Value> {
    let result: Value = serde_json::from_str(string)?;
    Ok(result)
}