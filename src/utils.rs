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

pub fn map_vec_to_struct_vec<T>(map:Vec<Map<String, Value>>) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
{
    let mut result:Vec<T> = Vec::new();
    for row in map{
        result.push(T::deserialize(json!(row))?);
    }
    Ok(result)
}