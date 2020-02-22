use std::io::{stdout, Write};

use actix_web::{App, get, HttpServer, Responder, web, HttpRequest,middleware,post};
use actix_web::HttpResponse;
use crossterm::{
    cursor, ExecutableCommand,
    QueueableCommand, Result, style::{self, Colorize}, terminal
};

use std::thread;

use log::{error, info, warn, trace, debug, LevelFilter};
use log4rs;
use std::time::Duration;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::encode::json::JsonEncoder;
use log4rs::config::{Appender, Root, Config};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use serde::{Deserialize, Serialize};

mod dto;
use dto::response;
use crate::dto::response::{testResp, Resp};

mod district;
use district::spider;
use crate::district::spider::{test_spider, test_http_client, fetch_district};

/**
road map:
practice
    1 get remote json data to mysql
    2 web api apply to some biz flow
    3 cross platform compile

concept:
    1 macro learning
**/

fn init_logger(){
    log4rs::init_file("log.yml", Default::default()).unwrap();
}

mod a{
    use log::{error, info, warn, trace, debug, LevelFilter};

    pub fn test(){
        error!("from mod a")
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    init_logger();

    let res=fetch_district().await;

    match res {
        Err(err) => println!("error : {}",err),
        Ok(dis)=>println!("ok")
    }

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(echo_json)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await

}

#[derive(Debug, Serialize, Deserialize)]
struct JsonObj {
    name: String,
    number: i32,
}

#[post("/test_json")]
async fn echo_json(item:web::Json<JsonObj>,req:HttpRequest) -> HttpResponse{

    let aa=1/0;

    debug!("request: {:?}",req);
    debug!("model: {:?}", item);

    HttpResponse::Ok().json(item.0)
}

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
//    format!("Hello {}! id:{}", info.1, info.0)
    HttpResponse::Ok().json("hahah")
}

#[get("/{test}/{name}/aaa.html")]
async fn aaa(info: web::Path<(String, String)>) -> HttpResponse{
//    Resp::ok("hahah").to_json_result().unwrap()
    HttpResponse::Ok().json("hahah")
}

