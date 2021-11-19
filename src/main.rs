use cuppa::database::DataBase;
use serde_json::{Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct User{
    id:i64,
    name:String,
}

fn main_async() -> Result<()>{
    let db = DataBase::new("localhost", 3306, "rust", "root", "");
    let rows = db.get_list("users".to_string(), "id = 5".to_string(), "10".to_string(), "id ASC".to_string(), "".to_string());
    println!("{:?}", rows);
    
    Ok(())
}

fn main() { 
    main_async().unwrap();
}