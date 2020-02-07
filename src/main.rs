use std::io::{stdout, Write};

use actix_web::{App, get, HttpServer, Responder, web};
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

mod dto;
use dto::response;
use crate::dto::response::{testResp, Resp};

//fn main()-> Result<()> {
//    println!("Hello, world!");
//
//    let mut stdout = stdout();
//
//    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
//
//    for y in 0..40 {
//        for x in 0..150 {
//            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
//                // in this loop we are more efficient by not flushing the buffer.
//                stdout
//                    .queue(cursor::MoveTo(x,y))?
//                    .queue(style::PrintStyledContent( "█".magenta()))?;
//            }
//        }
//    }
//    stdout.flush()?;
//    Ok(())
//
//
//}

fn init_logger(){
    log4rs::init_file("log.yml", Default::default()).unwrap();

//    loop {
//        thread::sleep(Duration::from_secs(1));
//        warn!("main");
//        error!("error main");
//        info!("a");
//        a::test();
//    }

}

fn mainTest(){

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
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await

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

