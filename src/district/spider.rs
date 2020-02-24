extern crate serde_json;


use hyper::Client;
use serde::Deserialize;
use serde::Serialize;
use bytes::{buf::BufExt};


pub fn test_spider() {
    println!("123")
}


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn test_http_client() -> Result<()> {
    let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();

    let users = fetch_json(url).await?;

    let sum = users.iter().fold(0, |acc, user| acc + user.id);

    println!("sum of ids : {}", sum);

    Ok(())
}

async fn fetch_json(url: hyper::Uri) -> Result<Vec<User>> {
    let client = Client::new();

    let res = client.get(url).await?;

    let body = hyper::body::aggregate(res).await?;

    let users = serde_json::from_reader(body.reader())?;

    Ok(users)
}

pub async fn fetch_district() -> Result<DistrictAPIDTO> {
    let url = "http://restapi.amap.com/v3/config/district?key=&keywords=&subdistrict=3".parse()?;

    let client = Client::new();

    let res = client.get(url).await?;

    let body = hyper::body::aggregate(res).await?;

    let district_info = serde_json::from_reader(body.reader())?;



    Ok(district_info)
}


#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct DistrictAPIDTO {
    status: String,
    info: String,
    infocode: String,
    suggestion: SuggestionDTO,
    districts: Vec<DistrictDTO>,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct SuggestionDTO {
    keywords: Vec<String>,
    cities: Vec<String>,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct DistrictDTO {
    // citycode: Vec<String>, //maybe array , maybe string , serde will occur error.
    adcode: String,
    name: String,
    center: String,
    level: String,
    districts: Vec<DistrictDTO>,
}

