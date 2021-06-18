# DataBase

```
// CREATE CONNECTION
let mut db = DataBase::new("localhost", 3306, "rust", "root", "");

// ADD: IF ROW EXIST UPDATE IT, OTHERWISE INSERT A NEW ROW
let mut data:serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    data.insert("name".to_string(), json!("Francisco"));
let row = db.update("users".to_string(), data, "id = 3".to_string(), "".to_string());
println!("{:?}", row);

// INSERT
let mut data:serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        data.insert("name".to_string(), json!("Francisco"));
let row = db.update("users".to_string(), data, "id = 3".to_string(), "".to_string());
println!("{:?}", row);

// UPDATE
let mut data:serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        data.insert("name".to_string(), json!("Francisco"));
let row = db.update("users".to_string(), data, "id = 3".to_string(), "".to_string());
println!("{:?}", row);

// GET ROW
let row = db.get_row("users".to_string(), "id = 99".to_string(), "".to_string(), "".to_string());
println!("{:?}", row);

// GET LIST
let rows = db.get_list("users".to_string(), "".to_string(), "10".to_string(), "id ASC".to_string(), "id, name".to_string());
println!("{:?}", rows);

// SQL QUERY
let rows = db.sql("SELECT * from users".to_string()).unwrap();
println!("{:?}", rows);
```