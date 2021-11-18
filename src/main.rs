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
    let row = db.get_row("users".to_string(), "id = 1".to_string(), "".to_string(), "".to_string());
    if row.is_ok(){
        let user:User = utils::map_to_struct(row.unwrap())?;
        println!("USER {:?}", user);
    }
    
    Ok(())
}

fn main() { 
    main_async().unwrap();
}