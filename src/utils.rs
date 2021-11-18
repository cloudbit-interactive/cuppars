/*
    [dependencies]
    serde_json = "1.0.64"
*/
use serde_json::{json, Result, Value, Map};
use serde::de::DeserializeOwned;
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

pub fn map_to_struct<T>(map:Map<String, Value>) -> Result<T>
    where
        T: DeserializeOwned,
{
    let value:Value = json!(map);
    T::deserialize(value)
}