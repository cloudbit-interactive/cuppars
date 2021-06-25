# DataBase

Add next dependencies to your Cargo.toml

``` toml
[dependencies]
mysql = "20.1.0"
serde_json = "1.0.64"
serde = { version = "1.0.126", features = ["derive"] }
serde_derive = "1.0.126"
```

Example

``` rust
use cuppa::database::DataBase;
use serde_json::{json, Map};

fn main() { 
    // CREATE CONNECTION
    let db = DataBase::new("localhost", 3306, "rust", "root", "");

    // ADD: IF ROW EXIST UPDATE IT, OTHERWISE INSERT A NEW ROW
    let mut data = Map::new();
        data.insert("name".to_string(), json!("Francisco"));
    let row = db.update("users".to_string(), data, "id = 3".to_string(), "".to_string());
    println!("{:?}", row);

    // INSERT
    let mut data = Map::new();
        data.insert("name".to_string(), json!("Francisco"));

    let row = db.update("users".to_string(), data, "id = 3".to_string(), "".to_string());
    println!("{:?}", row);

    // UPDATE
    let mut data = Map::new();
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
}
```