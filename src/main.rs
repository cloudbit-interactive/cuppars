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
    let rows = db.get_list("users".to_string(), "".to_string(), "10".to_string(), "id ASC".to_string(), "id, name".to_string());
    if rows.is_ok(){
        let result:Vec<User> = utils::map_vec_to_struct_vec(rows.unwrap())?;
        println!("USER {:?}", result);
    }
    
    Ok(())
}

fn main() { 
    main_async().unwrap();
}