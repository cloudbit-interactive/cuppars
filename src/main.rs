use cuppa::database::DataBase;
use serde_json::{json, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct User{
    id:i64,
    name:String,
}

fn main_async() -> Result<()>{
    let db = DataBase::new("localhost", 3306, "rust", "root", "");
    let row = db.get_row("users".to_string(), "id = 1".to_string(), "".to_string(), "".to_string());
    if row.is_err() { return Ok(()); }
    let row = row.unwrap();
    let user:User = serde_json::from_value(json!(row))?;
    println!("{:?}", user);
    Ok(())
}

fn main() { 
    main_async().unwrap();
}