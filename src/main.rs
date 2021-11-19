use cuppa::database::DataBase;
use cuppa::utils;
use serde_json::{Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct User{
    id:i64,
    name:String,
}

fn main_async() -> Result<()>{
    let db = DataBase::new("localhost", 3306, "rust", "root", "");
    let result = db.get_total_rows("users".to_string(), "".to_string());
    println!("{:?}", result);   
    
    Ok(())
}

fn main() { 
    main_async().unwrap();
}