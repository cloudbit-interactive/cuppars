use serde_json::{Result, Value};

/// # Example:
///
/// ```
/// let result = json_decode(r#"{"name":"Tufik", "age":99}"#).unwrap();
/// println!("{:?}", result["name"]);
/// ```
pub fn json_decode(string:&str) -> Result<Value> {
    let result: Value = serde_json::from_str(string)?;
    Ok(result)
}

pub fn test(string:&str) -> &str{
    return string;
}