/// # [dependencies]
/// mysql = "*"
/// serde_json = "1.0.64"
/// serde = { version = "1.0.126", features = ["derive"] }
/// serde_derive = "1.0.126"

use mysql::{Pool};
use mysql::prelude::*;
use serde_json::{json};
use std;

pub const RESULT_NOT_FOUND: &str = "RESULT_NOT_FOUND";

#[derive(Debug)]
pub struct DataBase{
    pub host:&'static str,
    pub port:u16,
    pub db:&'static str,
    pub user:&'static str,
    pub pass:&'static str,
    pub pool:mysql::Pool,
}

impl DataBase {
    
    /// # EXAMPLE
    /// ```
    /// let db = DataBase::new("localhost", 3306, "rust", "root", "");
    /// ```
    pub fn new(host: &'static str, port: u16, db: &'static str, user:&'static str, pass:&'static str) -> DataBase {
        let url = format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db);
        let pool = Pool::new(url).unwrap();
        let db = DataBase { host, port, db, user, pass, pool};
        db
    }

    /// Mix of Insert / Update: The method checks if a row already exists and update it; otherwise, insert a new record. Both cases return the affected row.
    /// 
    /// # EXAMPLE
    /// ```
    /// let mut data = Map::new();
    ///     data.insert("name".to_string(), json!("Zulay"));
    /// 
    /// let row = db.add("users".to_string(), data, "id = 333".to_string(), "".to_string());
    /// println!("{:?}", row);
    /// ```
    pub fn add(&self, table:String, data:serde_json::Map<String, serde_json::Value>, condition:String, columns:String)
    -> std::result::Result<serde_json::Map<String, serde_json::Value>, String>
    {
        let row = self.get_row(table.clone(), condition.clone(), "".to_string(), "".to_string());
        let result;
        if row.is_err(){
            result = self.insert(table, data, columns);
        }else{
            result = self.update(table, data, condition, columns);
        }

        if result.is_err() {
            return Err(format!("{}", result.unwrap_err()));
        }

        std::result::Result::Ok(result.clone().unwrap())
    }

    /// # Example
    ///
    /// ```
    /// let mut data = Map::new();
    ///     data.insert("name".to_string(), json!("Francisco"));
    /// 
    /// let row = db.insert("users".to_string(), data, "".to_string());
    /// println!("{:?}", row);
    /// ```
    pub fn insert(&self, table:String, data:serde_json::Map<String, serde_json::Value>, columns: String)
    -> std::result::Result<serde_json::Map<String, serde_json::Value>, String>
    {
        let mut keys:Vec<String> = Vec::new();
        let mut values:Vec<String> = Vec::new();
        for (key, value) in data {
            keys.push(key);
            let mut value_str = DataBase::json_value_to_string(value);
                    value_str = value_str.replace("'", "\'");
            if value_str != "NOW()" {
                value_str = format!("'{}'", value_str);
            }
            values.push(value_str);
        }
        let sql = format!("INSERT INTO {} (`{}`) VALUES ({})", table, keys.join("` , `"), values.join(" , "));
        let result = self.sql(sql);
        if result.is_err() {
            return Err(format!("{}", result.unwrap_err()));
        }

        let last_table_id = self.last_table_id(table.clone());
        let row = self.get_row(table, format!("id = {}", last_table_id), "".to_string(), columns);

        if row.is_err(){
            return Err(format!("{}", row.unwrap_err()));
        }

        if row.is_ok(){
            std::result::Result::Ok(row.clone().unwrap())
        }else{
            std::result::Result::Err(RESULT_NOT_FOUND.to_string())
        }
    }

    /// # Example
    ///
    /// ```
    /// let mut data = Map::new();
    ///     data.insert("name".to_string(), json!("Francisco"));
    ///
    /// let row = db.update("users".to_string(), data, "id = 1".to_string(), "".to_string());
    /// println!("{:?}", row);
    /// ```
    pub fn update(&self, table:String, data:serde_json::Map<String, serde_json::Value>, condition:String, columns:String)
    -> std::result::Result<serde_json::Map<String, serde_json::Value>, String>
    {
        let mut values:Vec<String> = Vec::new();
        for (key, value) in data {
            let mut value_str = DataBase::json_value_to_string(value);
                    //value_str = value_str.replace("'", "\'");
            if value_str != "NOW()" {
                value_str = format!("'{}'", value_str);
            }
            values.push(format!("{}={}", key, value_str));
        }
        let sql = format!("UPDATE {} SET {} WHERE {}", table, values.join(","), condition);
        let result = self.sql(sql);
        if result.is_err() {
            return Err(format!("{}", result.unwrap_err()));
        }

        let row = self.get_row(table, condition, "".to_string(), columns);

        if row.is_err(){
            return Err(format!("{}", row.unwrap_err()));
        }

        if row.is_ok(){
            std::result::Result::Ok(row.clone().unwrap())
        }else{
            std::result::Result::Err(RESULT_NOT_FOUND.to_string())
        }
    }

    /// # Example
    ///
    /// ```
    /// let rows = db.get_list("users".to_string(), "".to_string(), "10".to_string(), "id ASC".to_string(), "id, name".to_string());
    /// println!("{:?}", rows);
    /// ```
    pub fn get_list(&self, table:String, condition:String, limit:String, order_by:String, columns:String) 
    -> std::result::Result<Vec<serde_json::Map<String, serde_json::Value>>, String>
    {
        let mut sql = format!("SELECT * FROM {}", table);
        if columns != "" { sql = format!("SELECT {} FROM {}", columns, table); }
        if condition != "" { sql = format!("{} WHERE {}", sql, condition); }
        if order_by != "" { sql = format!("{} ORDER BY {}", sql, order_by); }
        if limit != "" { sql = format!("{} LIMIT {}", sql, limit); }

        let result = self.sql(sql);
        if result.is_err() {
            return Err(format!("{}", result.unwrap_err()));
        }

        let result = result.unwrap();
        Ok(result)
    } 

    /// # Example
    /// 
    /// ```
    /// let total_rows = db.get_total_rows("users".to_string(), "".to_string());
    /// println!("{:?}", total_rows);
    /// ```
    pub fn get_total_rows(&self, table:String, condition:String) 
    -> i32
    {
        let mut sql = format!("SELECT COUNT(*) AS total FROM {}", table);
        if condition != "" { sql = format!("{} WHERE {}", sql, condition); }

        let result = self.sql(sql);
        if result.is_err() {
            return 0;
        }

        let result = result.unwrap();
        let total = result[0]["total"].as_i64().unwrap();
        total as i32
    }

    /// # Example
    ///
    /// ```
    /// let row = db.get_row("users".to_string(), "id = 1".to_string(), "".to_string(), "".to_string());
    /// println!("{:?}", row);
    /// ```
    pub fn get_row(&self, table:String, condition: String, order_by: String, columns: String) 
    -> std::result::Result<serde_json::Map<String, serde_json::Value>, String>
    {
        let mut sql = format!("SELECT * FROM {}", table);
        if columns != "" { sql = format!("SELECT {} FROM {}", columns, table); }
        if condition != "" { sql = format!("{} WHERE {}", sql, condition); }
        if order_by != "" { sql = format!("{} ORDER BY {}", sql, order_by); }
        sql = format!("{} LIMIT 1", sql);

        let result = self.sql(sql);
        if result.is_err() {
            return Err(format!("{}", result.unwrap_err()));
        }
        
        let result = result.unwrap();
        if result.len() >= 1 {
            let row:serde_json::Map<String, serde_json::Value>;
                row = result[0].clone();
            Ok(row)
        }else{
            Err(RESULT_NOT_FOUND.to_string())
        }
    }

    /// # Example
    ///
    /// ```
    /// let rows = db.sql("SELECT * from users".to_string()).unwrap();
    /// println!("{:?}", rows);
    /// ```    
    pub fn sql(&self, sql:String)
    -> std::result::Result<Vec<serde_json::Map<String, serde_json::Value>>, String>
    {
        let result = self.__sql__(sql);
        if result.is_err() {
            std::result::Result::Err(format!("{}", result.unwrap_err()))
        } else {
            let result = result.unwrap();
            std::result::Result::Ok(result)
        }
    }

    fn __sql__(&self, sql:String)
    -> mysql::Result<Vec<serde_json::Map<String, serde_json::Value>>>
    {
        let mut conn = self.pool.get_conn().unwrap();
        let mut result:Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();
        let rows:Vec<mysql::Row> = conn.exec(sql, ())?;
        for i in 0..rows.len() {
            let map = self.row_to_map(rows[i].clone());
            result.push(map);
        }
        std::result::Result::Ok(result)
    }

    fn row_to_map(&self, row:mysql::Row) -> serde_json::Map<String, serde_json::Value>{
        let mut map = serde_json::Map::new();
        for column in row.columns_ref() {
            let column_value = &row[column.name_str().as_ref()];
            let column_name = column.name_str();
            match column_value {
                _val @ mysql::Value::NULL => {
                    map.insert(column_name.to_string(), json!(null));
                },
                val @ mysql::Value::Bytes(..) => {
                    let val:String = mysql::from_value(val.clone());
                    map.insert(column_name.to_string(), json!(val));
                }
                val @ mysql::Value::Int(..) => {
                    let val:i64 = mysql::from_value(val.clone());
                    map.insert(column_name.to_string(), json!(val));
                }
                val @ mysql::Value::UInt(..) => {
                    let val:u64 = mysql::from_value(val.clone());
                    map.insert(column_name.to_string(), json!(val));
                }
                val @ mysql::Value::Float(..) => {
                    let val:f32 = mysql::from_value(val.clone());
                    map.insert(column_name.to_string(), json!(val));
                }
                val @ mysql::Value::Double(..) => {
                    let val:f64 = mysql::from_value(val.clone());
                    map.insert(column_name.to_string(), json!(val));
                }
                val @ mysql::Value::Date(..) => {
                    let val:mysql::chrono::NaiveDateTime = mysql::from_value(val.clone());
                    let val = val.format("%Y-%m-%dT%H:%M:%S").to_string();
                    map.insert(column_name.to_string(), json!(val));
                }
                val @ mysql::Value::Time(..) => {
                    let val:std::time::Duration = mysql::from_value(val.clone());
                    let seconds = val.as_secs() % 60;
                    let minutes = (val.as_secs() / 60) % 60;
                    let hours = (val.as_secs() / 60) / 60;
                    let val = format!("{}:{}:{}", hours, minutes,seconds);
                    map.insert(column_name.to_string(), json!(val));
                }
            }
        }
        return map
    }

    pub fn last_table_id(&self, table:String)->i64{
        let result = self.sql(format!("SELECT MAX(id) AS last_id FROM {};", table));
        let mut last_id = 0;
        if result.is_ok(){
            last_id = result.unwrap()[0]["last_id"].as_i64().unwrap();
        }
        return last_id;
    }

    pub fn last_insert_id(&self)->i64{
        let result = self.sql("SELECT LAST_INSERT_ID() as last_insert_id".to_string());
        let mut last_insert_id = 0;
        if result.is_ok(){
            last_insert_id = result.unwrap()[0]["last_insert_id"].as_i64().unwrap();
        }
        return last_insert_id;
    }

        fn json_value_to_string(value:serde_json::Value) -> String{
            let result;
            if value.is_number() || value.is_f64() || value.is_i64() {
                result = value.as_f64().unwrap().to_string();
            }else if value.is_boolean() {
                if value.as_bool().unwrap() == true {
                    result = "1".to_string();
                }else{
                    result = "0".to_string();
                }
            }else{
                result = value.as_str().unwrap().to_string();
            }
            return result;
        }

}