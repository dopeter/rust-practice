use actix_web::HttpResponse;
use serde::{Serialize, Deserialize};
use failure::*;

pub fn testResp(){
    println!("123")
}


#[derive(Fail, Debug)]
pub enum BizException {
    #[fail(display = "Validation error on field: {}", field)]
    ValidationError { field: String },
    #[fail(display = "An internal error occurred. Please try again later.")]
    InternalError,
}

#[derive(Deserialize, Serialize)]
pub struct Resp<T> where T:Serialize{
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T:Serialize> Resp<T>{

    pub fn ok(data:T) -> Self {
        Resp{
            code:0,
            msg:"success".to_owned(),
            data:Some(data)
        }
    }

    pub fn to_json_result(&self) -> Result<HttpResponse,BizException>{
        Ok(HttpResponse::Ok().json(self))
    }

}

impl Resp<()>{

    pub fn err(error:i32,msg:&str) -> Self{
        Resp{
            code:error,
            msg:msg.to_owned(),
            data:None
        }
    }

}

