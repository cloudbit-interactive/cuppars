use cuppa::{json_decode};

fn main() { 
    let result = json_decode(r#"{"name":"Tufik", "age":99}"#).unwrap();
    println!("{:?}", result["name"]);
}