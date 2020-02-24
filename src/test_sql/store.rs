


use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use chrono::NaiveDateTime;
use diesel::Queryable;


use crate::*;
use self::schema::districts::dsl::*;
use self::schema::districts;



pub fn establish_connection() -> MysqlConnection{

    dotenv().ok();

    let database_url=env::var("DATABASE_URL").unwrap();

    MysqlConnection::establish(&database_url).expect(&format!("Error connection to {}",database_url))

}

pub fn test_sql(){

    let conn=establish_connection();

    // let dd:Result<District, diesel::result::Error>=districts::table.first(&conn);
    //
    // match dd {
    //     Err(err) => println!("error : {}",err),
    //     Ok(dis)=>{
    //
    //         println!("time is {}",dis.record_time.timestamp_millis());
    //
    //         println!("ok")
    //     }
    // }

    let res=districts
        // .filter(name.eq("aaaa")).limit(5)
        .load::<District>(&conn)
        .expect("Error");


    println!("size of dis is {}",res.len())

}

#[derive(Queryable)]
pub struct District{
    pub id:i32,
    pub name:String,
    pub code:Option<String>,
    pub parent_id:Option<i64>,
    pub record_time:NaiveDateTime
}