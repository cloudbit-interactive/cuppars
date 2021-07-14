use cuppa::database::DataBase;
use serde_json::{json, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct User{
    id:i64,
    name:String,
}

/*
#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
fn get_word_count_from_file(file_path:&str) -> Result<u32, &'static str>{
    if file_path == "" {
        return Result::Err("File can not be found!");
    }
    println!("\nget_word_count_from_file {}", file_path);
    Result::Ok(20)
}
*/

fn main() { 
    let db = DataBase::new("localhost", 3306, "rust", "root", "");
    let row = db.get_row("users".to_string(), "id = 1".to_string(), "".to_string(), "".to_string());
    if row.is_err() { return; }
    let row = row.unwrap();
    let user = || -> Result<User>{ Ok( serde_json::from_str( &json!(row).to_string())? ) }().unwrap();
    println!("{:?}", user);
}